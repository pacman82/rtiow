// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// Online ppm viewer: http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
mod camera;
mod hittable;
mod material;
mod output;
mod ray;
mod sample;
mod sphere;
mod vec3;
mod world;

use crate::{
    camera::Camera,
    output::save_image,
    sample::render_sample,
    vec3::{Color, Point, Vec3},
    world::create_world,
};
use indicatif::{ProgressBar, ProgressStyle};
use rand::thread_rng;
use rayon::prelude::*;
use structopt::StructOpt;

use std::{
    io,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, default_value = "100")]
    samples_per_pixel: u32,
    #[structopt(long, default_value = "50")]
    max_depth: u32,
    #[structopt(long, default_value = "384")]
    image_width: u32,
    #[structopt(long, short = "o", default_value = "image.png")]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    // Running is going to become `false` if execution is interrupted with Ctrl+C. In which case we
    // want to stop rendering and produce output asap.
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let Cli {
        samples_per_pixel,
        max_depth,
        image_width,
        output,
    } = Cli::from_args();

    let aspect_ratio = 16. / 9.;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let mut rng = thread_rng();

    let world = create_world(&mut rng);

    let lookfrom = Point::new(13., 2., 3.);
    let lookat = Point::new(0., 0., 0.);
    let distance_to_focus = 10.;

    let camera = Camera::new(
        20.,
        aspect_ratio,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        distance_to_focus,
        0.1,
    );

    eprintln!(
        "Start rendering samples. You can press Ctrl+C to finish rendering the current samples and \
        then produce output immediatly with samples rendered so far."
    );

    let progress_bar = ProgressBar::new(samples_per_pixel as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}][{eta}] {wide_bar} samples: {pos}/{len}"),
    );

    let neutral = || {
        (
            vec![Color::new(0., 0., 0.); image_height as usize * image_width as usize],
            0,
        )
    };
    // Render samples in parallel and reduce them to one accumulated color vector.
    let (mut acc_color_buf, num_samples_rendered) = (0..samples_per_pixel)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let sample = if running.load(Ordering::SeqCst) {
                (
                    render_sample(
                        &mut rng,
                        max_depth,
                        image_height,
                        image_width,
                        &camera,
                        &world,
                    ),
                    1,
                )
            } else {
                neutral()
            };
            progress_bar.inc(1);
            sample
        })
        .reduce(neutral, |(mut acc, acc_num), (sample, weight)| {
            for index in 0..acc.len() {
                acc[index] = acc[index] + sample[index];
            }
            (acc, acc_num + weight)
        });

    progress_bar.finish();

    if num_samples_rendered == 0 {
        eprintln!("No samples rendered.")
    } else {
        let samples_f = num_samples_rendered as f64;

        for color in &mut acc_color_buf {
            *color = *color / samples_f;
        }

        save_image(&acc_color_buf, image_width, &output)?;

        eprintln!("Done.");
    }

    Ok(())
}

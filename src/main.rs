// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// Online ppm viewer: http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html
mod bounding_box;
mod bvh;
mod camera;
mod hittable;
mod material;
mod moving;
mod output;
mod persistence;
mod ray;
mod scene;
mod shape;
mod vec3;
mod random_scenes;
mod texture;
mod renderable;
mod perlin;

use crate::{output::save_image, persistence::SceneBuilder, vec3::Color};
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

/// A ray tracer based on the methods presented in the Ray Tracing in one Weekend tutorial.
#[derive(StructOpt)]
struct Cli {
    /// Number of rays calculated for each pixel. Larger numbers produce smoother and less dotty
    /// pictures, but calculation time increases linear with larger numbers.
    #[structopt(long, default_value = "100")]
    samples_per_pixel: u32,
    /// Maximum number of "bounces" calculated for each Ray.
    #[structopt(long, default_value = "50")]
    max_depth: u32,
    /// Horizontal width of the picture in pixels.
    /// Aspect ratio.
    #[structopt(long, default_value = "384")]
    image_width: u32,
    /// Vertical width of the picture in pixels.
    #[structopt(long, default_value = "216")]
    image_width: u32,
    /// Path to a JSON file describing the Scene to be rendered. If no value is given a picture with
    /// mostly random spheres is used.
    #[structopt(long, short = "i")]
    input: Option<PathBuf>,
    /// The rendered Scene is going to be saved in this file.
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
        image_height,
        image_width,
        input,
        output,
    } = Cli::from_args();

    let aspect_ratio = image_width as f64 / image_height as f64;

    let mut rng = thread_rng();

    let scene = if let Some(path) = input {
        SceneBuilder::from_path(path)?
    } else {
        let scene = random_scenes::spheres(&mut rng, aspect_ratio);
        eprintln!("No input scene specified. Saving scene with random spheres to 'scene.json'.");
        scene.to_path("scene.json")?;
        scene
    }
    .build();

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
                    scene.render_sample(
                        &mut rng,
                        max_depth,
                        image_height,
                        image_width,
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

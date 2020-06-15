use crate::Color;
use image::ImageBuffer;
use std::{io, path::Path};

pub fn save_image(color_buf: &Vec<Color>, image_width: u32, output: &Path) -> io::Result<()> {
    let mut image_buffer =
        ImageBuffer::new(image_width, (color_buf.len() / image_width as usize) as u32);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let rgb = gamma_corrected_rgb(&color_buf[(y * image_width + x) as usize]);
        *pixel = image::Rgb(rgb);
    }

    match image_buffer.save(output) {
        Err(image::ImageError::IoError(e)) => return Err(e),
        Err(image::ImageError::Unsupported(e)) => {
            if output.extension().unwrap_or_default() != "png" {
                eprintln!("{}", e);
                // Unsupported file extension.
                eprintln!("I'll try to save into an `.png` file instead.");
                let mut new_path = output.to_path_buf();
                new_path.set_extension("png");
                save_image(color_buf, image_width, &new_path)?;
            } else {
                panic!("Unexpected error saving to image file: {}", e)
            }
        }
        Err(e) => panic!("Unexpected error saving to image file: {}", e),
        Ok(()) => (),
    }
    Ok(())
}

fn gamma_corrected_rgb(color: &Color) -> [u8; 3] {
    let red = (256. * clamp(color[0].sqrt(), 0., 0.999)) as u8;
    let green = (256. * clamp(color[1].sqrt(), 0., 0.999)) as u8;
    let blue = (256. * clamp(color[2].sqrt(), 0., 0.999)) as u8;
    [red, green, blue]
}

fn clamp(f: f64, low: f64, high: f64) -> f64 {
    if f < low {
        low
    } else if f > high {
        high
    } else {
        f
    }
}

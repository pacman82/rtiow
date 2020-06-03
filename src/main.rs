// http://cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html

use std::{io, ops::Deref};

fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    // P3 means colors are in ASCII. Then width, then height. 255 is the max color.
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).into_iter().rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            pixel_color.write_color(io::stdout().lock())?;
        }
    }
    eprintln!("Done!");
    Ok(())
}

type Color = Vec3;

struct Vec3([f64; 3]);

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    fn write_color(&self, mut out: impl io::Write) -> io::Result<()> {
        let red = (255.999 * self[0]) as i32;
        let green = (255.999 * self[1]) as i32;
        let blue = (255.999 * self[2]) as i32;

        // Now print RGB tripplets
        writeln!(out, "{} {} {}", red, green, blue)
    }
}

impl Deref for Vec3 {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

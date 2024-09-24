use clap::Parser;
use image::{RgbImage, Rgb};
// use rayon::prelude::*;

mod vec;
use vec::{Vec3, Color};


// See https://github.com/heyjuvi/raytracinginrust

/// `glint`, a simple raytracer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image width
    #[arg(short='W', long, default_value_t = 256)]
    width: u32,

    /// Image height
    #[arg(short='H', long, default_value_t = 256)]
    height: u32,

    /// Output path
    #[arg(short, long, default_value_t = String::from("image.png"))]
    outpath: String
}





fn main() {
    // parse arguments
    let args = Args::parse();

    // set up an ImageBuffer of Pixels
    let mut img = RgbImage::new(args.width, args.height);

    // use par_enumerate_pixels_mut for parallel execution
    img.enumerate_pixels_mut()
        .for_each(|(i, jj, pixel)|{
            let j = 255 - jj;

            let px_color = Color::new(
                (i as f64) / ((args.width - 1) as f64),
                (j as f64) / ((args.height - 1) as f64),
                0.25
            );

            // NOTE: PNG origin is (0,0) -> Top Left
            //       flip it so the img coords are
            //       aligned with the camera
            *pixel = px_color.to_rgb();

        });

    img.save(args.outpath).unwrap();
}

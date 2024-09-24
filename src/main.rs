use image::{RgbImage, Rgb};
use std::path::Path;
// use rayon::prelude::*;

fn main() {

    const IMAGE_WIDTH:  u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    let img_path = Path::new("image.png");

    // set up an ImageBuffer of Pixels
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);


    // Render Loop (can we parallelize this?)
    // https://users.rust-lang.org/t/getting-artifacts-while-making-the-ray-tracing-parallel/106981/11
    // img.par_enumerate_pixels_mut()
    //     .for_each(|(i, jj, pixel)| {
    //         let j = 255 - jj;

    //         let r = 255.999 * (i as f64) / ((IMAGE_WIDTH - 1) as f64);
    //         let g = 255.999 * (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
    //         let b = 255.999 * 0.25;


    //         // NOTE: PNG origin is (0,0) -> Top Left
    //         //       flip it so the img coords are
    //         //       aligned with the camera
    //         *pixel = Rgb([r as u8, g as u8, b as u8]);
    //     });


    img.enumerate_pixels_mut()
        .for_each(|(i, jj, pixel)|{
            let j = 255 - jj;

            let r = 255.999 * (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = 255.999 * (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 255.999 * 0.25;


            // NOTE: PNG origin is (0,0) -> Top Left
            //       flip it so the img coords are
            //       aligned with the camera
            *pixel = Rgb([r as u8, g as u8, b as u8]);

        });

    img.save(img_path).unwrap();
}

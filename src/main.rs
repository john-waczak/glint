use clap::Parser;
use image::RgbImage;
// use rayon::prelude::*;

mod vec;
mod ray;

use vec::{Vec3, Point3, Color};
use ray::Ray;


// See https://github.com/heyjuvi/raytracinginrust

/// `glint`, a simple raytracer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image width
    #[arg(short='W', long, default_value_t = 256)]
    width: u32,

    /// Image height
    #[arg(short='H', long, default_value_t = 144)]
    height: u32,

    /// Output path
    #[arg(short, long, default_value_t = String::from("image.png"))]
    outpath: String
}



fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}


fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().normalized();
    let s = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - s) * Color::new(1.0, 1.0, 1.0) + s * Color::new(0.5, 0.7, 1.0)
}


fn main() {
    // parse arguments
    let args = Args::parse();

    // set up an ImageBuffer of Pixels
    let mut img = RgbImage::new(args.width, args.height);

    // set up viewport
    let aspect_ratio = (args.width as f64) / (args.height as f64);
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);



    
    // use par_enumerate_pixels_mut for parallel execution
    img.enumerate_pixels_mut()
        .for_each(|(i, jj, pixel)|{
            // NOTE: PNG origin is (0,0) -> Top Left
            //       flip it so the img coords are
            //       aligned with the camera
            let j = args.height - jj;

            let u = (i as f64) / ((args.width - 1) as f64);
            let v = (j as f64) / ((args.height- 1) as f64);

            let r = Ray::new(origin, lower_left_corner + (u * horizontal) + (v * vertical) - origin);

            let pixel_color = ray_color(&r);
            *pixel = pixel_color.to_rgb();
        });

    img.save(args.outpath).unwrap();
}

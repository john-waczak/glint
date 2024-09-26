use clap::Parser;
use image::RgbImage;
use rand::prelude::*;

mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;

use vec::{Vec3, Point3, Color};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;


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

    /// Samples per pixel
    #[arg(short='s', long, default_value_t = 100)]
    samples_per_pixel: u64,

    /// Output path
    #[arg(short, long, default_value_t = String::from("image.png"))]
    outpath: String
}



fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}


fn main() {
    // parse arguments
    let args = Args::parse();

    // set up an ImageBuffer of Pixels
    let mut img = RgbImage::new(args.width, args.height);

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new(args.width, args.height);

    let mut rng = rand::thread_rng();
    // use par_enumerate_pixels_mut for parallel execution
    img.enumerate_pixels_mut()
        .for_each(|(i, jj, pixel)|{
            // NOTE: PNG origin is (0,0) -> Top Left
            //       flip it so the img coords are
            //       aligned with the camera
            let j = args.height - jj;

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..args.samples_per_pixel {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((args.width - 1) as f64);
                let v = ((j as f64) + random_v) / ((args.height- 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            *pixel = pixel_color.to_rgb(args.samples_per_pixel);
        });

    img.save(args.outpath).unwrap();
}

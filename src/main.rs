use clap::Parser;
use image::RgbImage;
use rand::prelude::*;
use std::rc::Rc;

mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use vec::{Point3, Color};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Specular};

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

    /// Samples per pixel
    #[arg(short='b', long, default_value_t = 5)]
    max_bounces: u64,



    /// Output path
    #[arg(short, long, default_value_t = String::from("image.png"))]
    outpath: String
}



fn ray_color(r: &Ray, world: &World, bounces_left: u64) -> Color {
    if bounces_left <= 0 {
        return Color::new(0.0, 0.0, 0.0); // if we hit nothing, return black
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, bounces_left - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
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

    // Materials
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Specular::new(Color::new(0.8, 0.8, 0.8)));
    let mat_right = Rc::new(Specular::new(Color::new(0.8, 0.6, 0.2)));

    // World
    let mut world = World::new();
    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

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
                pixel_color += ray_color(&r, &world, args.max_bounces);
            }

            *pixel = pixel_color.to_rgb(args.samples_per_pixel);
        });

    img.save(args.outpath).unwrap();
}

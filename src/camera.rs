use super::vec::{Vec3, Point3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3
}

impl Camera {
    pub fn new(img_width: u32, img_height: u32) -> Camera {
        let aspect_ratio = (img_width as f64) / (img_height as f64);
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let orig = Point3::new(0.0, 0.0, 0.0);
        let h = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, viewport_height, 0.0);
        let llc = orig - (h / 2.0) - (v / 2.0) - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin: orig,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin
        )
    }
}

use super::vec::{Vec3, Color};
use super::ray::Ray;
use super::hit::HitRecord;

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}



// -------- LAMBERTIAN -------------------------------------------------


pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}


impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // sample scattered direction as direction on unit sphere
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();

        // handle near zero-vector case
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}







// ---------- SPECULAR --------------------------------------------------
pub struct Specular {
    albedo: Color
}

impl Specular {
    pub fn new(albedo: Color) -> Specular {
        Specular {
            albedo
        }
    }
}

impl Scatter for Specular {
    fn scatter(&self, r_in :&Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

use super::vec::{Vec3, Color};
use super::ray::Ray;
use super::hit::HitRecord;
use rand::prelude::*;

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
    albedo: Color,
    fuzz: f64,
}

impl Specular {
    pub fn new(albedo: Color, fuzz: f64) -> Specular {
        Specular {
            albedo,
            fuzz
        }
    }
}

impl Scatter for Specular {
    fn scatter(&self, r_in :&Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}



// ---------- DIELECTRIC --------------------------------------------------

pub struct Dielectric {
    idx_ref: f64  // index of refraction
}

impl Dielectric {
    pub fn new(idx_ref: f64) -> Dielectric {
        Dielectric {
            idx_ref
        }
    }


    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}


impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.idx_ref
        } else {
            self.idx_ref
        };

        let unit_direction = r_in.direction().normalized();

        let cos_theta = ((-1.0) * unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0; // TIR
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

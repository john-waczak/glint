use super::vec::{Vec3, Point3};

pub struct Ray {
    orig: Point3,
    dir: Vec3
}


impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            orig,
            dir,
        }
    }


    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, s: f64) -> Point3 {
        self.orig + s * self.dir
    }
}

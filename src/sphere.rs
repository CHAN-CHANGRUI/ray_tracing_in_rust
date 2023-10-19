use std::rc::Rc;

use crate::{hittable::*, interval::Interval, material::Material, ray::*, vec3::*};


#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat: mat.clone(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord::default();
        hit_record.t = root;
        hit_record.p = r.point_at_parameter(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        hit_record.mat = Some(Rc::clone(&self.mat));

        Some(hit_record)
    }
}

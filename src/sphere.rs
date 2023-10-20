

use crate::{hittable::*, interval::Interval, ray::*, vec3::*, material};


#[derive(Clone)]
pub struct Sphere<Material> {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Material,
}

impl <Material>Sphere<Material> {
    pub fn new(center: Vec3, radius: f64, mat: Material) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl <Material: Sync + material::Material>Hittable for Sphere <Material> {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.point_at_parameter(t);
        let outward_normal = (p - self.center) / self.radius;
        let (normal, front_face) = HitRecord::set_face_normal(r, outward_normal);
        

        Some(HitRecord { p, normal, t, front_face, mat: &self.mat })
    }
}

use super::vector::*;
use super::hittable::*;
use super::ray::*;

#[derive(Debug)]
pub struct Sphere{
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: MatPtr
}
impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let oc = r.origin() - self.center;
        let a = r.direction().sqrlen();
        let half_b = dot(oc, r.direction());
        let c = oc.sqrlen() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 { return None } 

        let sqrtd = discriminant.sqrt();

        // this finds the nearest root that lies in the range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root{
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root{
                return None;
            }
        }
        let p = r.at(root);
        let normal = (p - self.center) / self.radius;
        let front_face = dot(r.dir, normal) < 0.0;

        Some(HitRecord{
            p,
            t: root,
            normal: if front_face { normal } else { -normal },
            mat_ptr: self.mat_ptr.clone(),
            front_face: front_face,
        })

        /*rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();
        true*/
    }
}

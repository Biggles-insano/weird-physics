// src/aabb.rs
use crate::math::Vec3;
use crate::ray::Ray;

pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub mat_id: usize,
}

#[derive(Copy, Clone)]
pub struct Aabb { pub min: Vec3, pub max: Vec3, pub mat_id: usize }

impl Aabb {
    pub fn hit(&self, ray: &Ray, mut tmin: f32, mut tmax: f32) -> Option<Hit> {
        // X axis
        let inv_dx = 1.0 / ray.d.x;
        let mut tx0 = (self.min.x - ray.o.x) * inv_dx;
        let mut tx1 = (self.max.x - ray.o.x) * inv_dx;
        if inv_dx < 0.0 { std::mem::swap(&mut tx0, &mut tx1); }
        tmin = tmin.max(tx0);
        tmax = tmax.min(tx1);
        if tmax <= tmin { return None; }

        // Y axis
        let inv_dy = 1.0 / ray.d.y;
        let mut ty0 = (self.min.y - ray.o.y) * inv_dy;
        let mut ty1 = (self.max.y - ray.o.y) * inv_dy;
        if inv_dy < 0.0 { std::mem::swap(&mut ty0, &mut ty1); }
        tmin = tmin.max(ty0);
        tmax = tmax.min(ty1);
        if tmax <= tmin { return None; }

        // Z axis
        let inv_dz = 1.0 / ray.d.z;
        let mut tz0 = (self.min.z - ray.o.z) * inv_dz;
        let mut tz1 = (self.max.z - ray.o.z) * inv_dz;
        if inv_dz < 0.0 { std::mem::swap(&mut tz0, &mut tz1); }
        tmin = tmin.max(tz0);
        tmax = tmax.min(tz1);
        if tmax <= tmin { return None; }

        let t = tmin;
        let p = ray.at(t);

        // normal por cara (epsilon)
        let eps = 1e-3;
        let mut n = Vec3::new(0.0, 0.0, 0.0);
        if (p.x - self.min.x).abs() < eps { n = Vec3::new(-1.0, 0.0, 0.0); }
        else if (p.x - self.max.x).abs() < eps { n = Vec3::new( 1.0, 0.0, 0.0); }
        else if (p.y - self.min.y).abs() < eps { n = Vec3::new(0.0,-1.0, 0.0); }
        else if (p.y - self.max.y).abs() < eps { n = Vec3::new(0.0, 1.0, 0.0); }
        else if (p.z - self.min.z).abs() < eps { n = Vec3::new(0.0, 0.0,-1.0); }
        else if (p.z - self.max.z).abs() < eps { n = Vec3::new(0.0, 0.0, 1.0); }

        Some(Hit { t, p, n, mat_id: self.mat_id })
    }
}
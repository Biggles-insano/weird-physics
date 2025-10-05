// src/camera.rs
use crate::math::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub u: Vec3, pub v: Vec3, pub w: Vec3, // base de cámara
    pub half_w: f32, pub half_h: f32,      // fov
}

impl Camera {
    pub fn from_euler(eye: Vec3, yaw_deg:f32, pitch_deg:f32, roll_deg:f32, fov_deg:f32, aspect:f32)->Self{
        let (yaw, pitch, roll) = (yaw_deg.to_radians(), pitch_deg.to_radians(), roll_deg.to_radians());
        let (cy, sy) = (yaw.cos(), yaw.sin());
        let (cx, sx) = (pitch.cos(), pitch.sin());
        let (cz, sz) = (roll.cos(), roll.sin());

        // Rz * Rx * Ry aplicado a ejes canónicos
        let right = Vec3::new(
            cz*cy + sz*sx*sy,  sz*cx,  cz*(-sy) + sz*sx*cy
        ).norm();
        let up = Vec3::new(
            -sz*cy + cz*sx*sy,  cz*cx, -sz*(-sy) + cz*sx*cy
        ).norm();
        let forward = Vec3::new(-cx*sy, sx, -cx*cy).norm();

        let w = (-forward).norm(); // convención clásica
        let u = right;
        let v = up;

        let theta = fov_deg.to_radians();
        let half_h = (theta*0.5).tan();
        let half_w = aspect * half_h;
        Self { origin: eye, u, v, w, half_w, half_h }
    }

    pub fn ray_for(&self, x:f32, y:f32)->crate::ray::Ray{
        // x,y en [-1,1]
        let dir = (self.u * (x*self.half_w) + self.v * (y*self.half_h) - self.w).norm();
        crate::ray::Ray{ o: self.origin, d: dir }
    }
}
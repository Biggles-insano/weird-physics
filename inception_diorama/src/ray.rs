// src/ray.rs
use crate::math::Vec3;
#[derive(Copy, Clone, Debug)]
pub struct Ray { pub o: Vec3, pub d: Vec3 }
impl Ray { pub fn at(&self, t:f32)->Vec3{ self.o + self.d*t } }
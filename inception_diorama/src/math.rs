// src/math.rs
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

impl Vec3 {
    pub fn new(x:f32,y:f32,z:f32)->Self{Self{x,y,z}}
    pub fn dot(self, o:Self)->f32{ self.x*o.x + self.y*o.y + self.z*o.z }
    pub fn len(self)->f32{ self.dot(self).sqrt() }
    pub fn norm(self)->Self{ let l=self.len().max(1e-8); Self::new(self.x/l,self.y/l,self.z/l)}
    pub fn clamp01(self)->Self{ Self::new(self.x.clamp(0.0,1.0), self.y.clamp(0.0,1.0), self.z.clamp(0.0,1.0)) }
}

pub fn v(a:f32)->Vec3{ Vec3::new(a,a,a) }

use std::ops::*;
impl Add for Vec3{ type Output=Self; fn add(self,o:Self)->Self{Self::new(self.x+o.x,self.y+o.y,self.z+o.z)}}
impl Sub for Vec3{ type Output=Self; fn sub(self,o:Self)->Self{Self::new(self.x-o.x,self.y-o.y,self.z-o.z)}}
impl Mul<f32> for Vec3{ type Output=Self; fn mul(self,s:f32)->Self{Self::new(self.x*s,self.y*s,self.z*s)}}
impl Mul<Vec3> for Vec3{ type Output=Self; fn mul(self,o:Self)->Self{Self::new(self.x*o.x,self.y*o.y,self.z*o.z)}}
impl Div<f32> for Vec3{ type Output=Self; fn div(self,s:f32)->Self{Self::new(self.x/s,self.y/s,self.z/s)}}
impl Neg for Vec3{ type Output=Self; fn neg(self)->Self{Self::new(-self.x,-self.y,-self.z)}}

pub fn reflect(i:Vec3, n:Vec3)->Vec3 { i - n * (2.0*i.dot(n)) }

pub fn refract(i:Vec3, n:Vec3, eta:f32)->Option<Vec3>{
    // i: incidente normalizado, n: normal normalizada, eta: n1/n2
    let cosi = (-i.dot(n)).clamp(-1.0, 1.0);
    let k = 1.0 - eta*eta*(1.0 - cosi*cosi);
    if k < 0.0 { None } else { Some(i*eta + n*(eta*cosi - k.sqrt())) }
}

pub fn lerp(a:Vec3, b:Vec3, t:f32)->Vec3{ a*(1.0-t) + b*t }

pub fn fresnel_schlick(cos_theta: f32, f0: Vec3) -> Vec3 {
    f0 + (v(1.0)-f0) * (1.0 - cos_theta).powf(5.0)
}
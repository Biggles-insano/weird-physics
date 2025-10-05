// src/material.rs
use crate::math::{Vec3, v, reflect, refract, fresnel_schlick, lerp};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub enum Kind {
    Diffuse,
    Metal { rough: f32 },
    Dielectric { ior: f32, absorption: Vec3 }, // refracción
    Emissive { intensity: f32 },
    Plastic { rough: f32 }, // difuso + specular
}

#[derive(Copy, Clone)]
pub struct Material {
    pub kind: Kind,
    pub albedo: Vec3,
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
}

pub fn shade(hit_p:Vec3, n:Vec3, ray:&Ray, mat:&Material, sky:&dyn Fn(Vec3)->Vec3, depth:i32)->Vec3 {
    let view = (-ray.d).norm();
    let cos_theta = n.dot(view).max(0.0);
    let f0 = lerp(v(0.04), mat.albedo, mat.reflectivity.clamp(0.0,1.0));
    let fres = fresnel_schlick(cos_theta, f0).clamp01();

    match mat.kind {
        Kind::Emissive { intensity } => mat.albedo * intensity,
        Kind::Diffuse => {
            // lambert + un poco de fresnel especular fake
            let base = mat.albedo * cos_theta;
            let spec = v(1.0) * fres.x; // aprox
            (base*0.9 + spec*0.1).clamp01()
        }
        Kind::Metal { rough } => {
            if depth <= 0 { return v(0.0); }
            let refl_dir = reflect(ray.d, n).norm();
            // “roughness” barato: mezcla con un poco la normal
            let jitter = n * (rough*0.2);
            let rd = (refl_dir + jitter).norm();
            let rray = Ray { o: hit_p + n*1e-3, d: rd };
            sky(rray.d) * fres.x
        }
        Kind::Dielectric { ior, absorption } => {
            if depth <= 0 { return v(0.0); }
            let entering = ray.d.dot(n) < 0.0;
            let (n1, n2, nn) = if entering {(1.0, ior, n)} else {(ior, 1.0, -n)};
            let eta = n1 / n2;
            let refr = refract(ray.d, nn, eta);
            let refl_dir = reflect(ray.d, nn).norm();
            let refl_col = sky(refl_dir);

            match refr {
                Some(td) => {
                    let trans_ray = Ray { o: hit_p - nn*1e-3, d: td.norm() };
                    // atenuación (Beer-Lambert)
                    let dist = 1.0; // corto salto
                    let att = Vec3::new(
                        (-absorption.x*dist).exp(),
                        (-absorption.y*dist).exp(),
                        (-absorption.z*dist).exp()
                    );
                    let trans_col = sky(trans_ray.d) * att;
                    (refl_col * fres + trans_col * (v(1.0)-fres)) * mat.albedo
                }
                None => {
                    // reflexión total
                    refl_col * mat.albedo
                }
            }
        }
        Kind::Plastic { rough } => {
            // difuso + specular Fresnel; “rough” solo atenúa el spec
            let lambert = mat.albedo * cos_theta;
            let spec = v(1.0) * fres * (1.0 - rough).max(0.0);
            (lambert*0.85 + spec*0.15).clamp01()
        }
    }
}
pub fn default_materials() -> Vec<Material> {
    vec![
        Material { // 0 piso difuso gris
            kind: Kind::Diffuse,
            albedo: Vec3::new(0.8, 0.8, 0.85),
            specular: 0.1,
            transparency: 0.0,
            reflectivity: 0.0,
        },
        Material { // 1 metal pulido
            kind: Kind::Metal { rough: 0.2 },
            albedo: Vec3::new(0.9, 0.9, 0.95),
            specular: 1.0,
            transparency: 0.0,
            reflectivity: 0.9,
        },
        Material { // 2 vidrio
            kind: Kind::Dielectric { ior: 1.5, absorption: Vec3::new(0.1, 0.03, 0.01) },
            albedo: Vec3::new(1.0, 1.0, 1.0),
            specular: 0.04,
            transparency: 1.0,
            reflectivity: 0.04,
        },
        Material { // 3 plástico rojo
            kind: Kind::Plastic { rough: 0.4 },
            albedo: Vec3::new(0.9, 0.2, 0.25),
            specular: 0.2,
            transparency: 0.0,
            reflectivity: 0.04,
        },
        Material { // 4 emisivo blanco
            kind: Kind::Emissive { intensity: 4.0 },
            albedo: Vec3::new(0.9, 0.9, 1.0),
            specular: 0.0,
            transparency: 0.0,
            reflectivity: 0.0,
        },
    ]
}
// src/scene.rs
use crate::math::{Vec3, lerp};
use crate::ray::Ray;
use crate::aabb::{Aabb, Hit};
use crate::material::Material;

pub struct Scene {
    pub cubes: Vec<Aabb>,
    pub mats: Vec<Material>,
}

impl Scene {
    pub fn test_scene()->Self{
        // materiales (5 diferentes)
        let mats = crate::material::default_materials();

        // cubos (plataforma + ciudad de edificios)
        let mut cubes: Vec<Aabb> = Vec::new();

        // 1) Plataforma/piso amplio
        cubes.push(Aabb{ min:Vec3::new(-12.0,-2.0,-12.0), max:Vec3::new( 12.0,-1.5, 12.0), mat_id:0 }); // piso difuso

        // 2) Grid de edificios (procedural). Espaciado y alturas variables.
        //    Evitamos el centro (plaza) para dejar la composición limpia.
        let spacing = 3.2_f32;    // distancia entre torres
        let half_n: i32 = 3;           // genera (2*half_n+1)^2 celdas
        for gx in -half_n..=half_n {
            for gz in -half_n..=half_n {
                if gx.abs() <= 1 && gz.abs() <= 1 { continue; } // deja plaza central

                let x = gx as f32 * spacing;
                let z = gz as f32 * spacing;

                // altura y material a partir de un hash estable
                let gu = (gx + 100) as u32;
                let zu = (gz + 200) as u32;
                let h = hash_u32(gu ^ zu.wrapping_mul(0x9E37_79B9));
                let hf = (h & 1023) as f32 / 1023.0; // 0..1
                let height = 0.8 + hf * 4.2;         // 0.8..5.0 aprox

                // asigna materiales variados (metal, plástico, vidrio, difuso)
                let mat_id = match h % 4 {
                    0 => 1, // metal
                    1 => 3, // plástico
                    2 => 2, // vidrio
                    _ => 0, // difuso
                } as usize;

                let half = 0.9_f32; // grosor de cada torre
                cubes.push(Aabb{
                    min: Vec3::new(x - half, -1.5, z - half),
                    max: Vec3::new(x + half, -1.5 + height * 1.6, z + half),
                    mat_id,
                });
            }
        }

        // 3) Elementos centrales (composición focal)
        // Columna metálica izquierda
        cubes.push(Aabb{ min:Vec3::new(-3.5,-1.5,-1.2), max:Vec3::new(-1.8, 2.0, 1.2),  mat_id:1 });
        // Columna plástica derecha
        cubes.push(Aabb{ min:Vec3::new( 1.8,-1.5,-1.0), max:Vec3::new( 3.2, 1.8, 1.0),  mat_id:3 });
        // Cubo de vidrio suspendido al centro
        cubes.push(Aabb{ min:Vec3::new(-0.8, 0.2,-0.8), max:Vec3::new( 0.8, 1.8, 0.8),  mat_id:2 });
        // Lámpara emisiva elevada
        cubes.push(Aabb{ min:Vec3::new(-0.5, 2.8,-0.5), max:Vec3::new( 0.5, 3.8, 0.5),  mat_id:4 });
        // Bloque metálico bajo al fondo para reflejos
        cubes.push(Aabb{ min:Vec3::new(-2.0,-1.5,-5.0), max:Vec3::new( 2.0,-0.2,-3.0),  mat_id:1 });

        // 4) Duplicado estilo Inception: ciudad "plegada" 90° (rota en X) y elevada
        // Rotación 90° en X: (y,z) -> (-z, y).  Luego trasladamos en Z' + 12.0 para que cuelgue arriba.
        let base_copy = cubes.clone();
        for c in base_copy {
            let min = c.min;
            let max = c.max;
            // Tras rotar 90° en X, las extremas en Y/Z se intercambian con signo en Y'.
            // Reajustamos AABB usando combinaciones de min/max correspondientes.
            let rot_min = Vec3::new(
                min.x,
                -max.z,
                min.y + 12.0
            );
            let rot_max = Vec3::new(
                max.x,
                -min.z,
                max.y + 12.0
            );
            cubes.push(Aabb { min: rot_min, max: rot_max, mat_id: c.mat_id });
        }
        Self { cubes, mats }
    }

    pub fn sky(&self, d:Vec3)->Vec3{
        // cielo negro con estrellas simples
        let t = 0.5*(d.y+1.0).clamp(0.0,1.0);
        let base = lerp(Vec3::new(0.0,0.0,0.0), Vec3::new(0.02,0.02,0.03), t);
        // estrellas discretas por hash angular (baratas)
        let phi = d.x.atan2(d.z) + std::f32::consts::PI; // 0..2pi
        let theta = (d.y.clamp(-1.0,1.0)).acos();        // 0..pi
        let u = (phi / (2.0*std::f32::consts::PI) * 1024.0).floor() as u32;
        let v = (theta / std::f32::consts::PI * 512.0).floor() as u32;
        let h = hash_u32(u ^ (v.wrapping_mul(0x9E37_79B9)));
        let star = ((h & 255) as f32 / 255.0).powf(36.0); // puntitos más finos
        base + Vec3::new(1.0,0.95,0.9) * (star*0.9)
    }

    pub fn trace(&self, ray:&Ray, depth:i32)->Vec3{
        let mut best: Option<Hit> = None;
        let mut far = 1e9;

        for c in &self.cubes {
            if let Some(h) = c.hit(ray, 0.001, far) {
                far = h.t;
                best = Some(h);
            }
        }

        match best {
            None => self.sky(ray.d),
            Some(h) => {
                let mat = &self.mats[h.mat_id];
                let sky_fn = |d:Vec3| self.sky(d);
                crate::material::shade(h.p, h.n, ray, mat, &sky_fn, depth-1)
            }
        }
    }
}

// hash baratísimo
fn hash_u32(mut x: u32) -> u32 {
    x ^= x >> 17; x = x.wrapping_mul(0xed5ad4bb);
    x ^= x >> 11; x = x.wrapping_mul(0xac4c1b51);
    x ^= x >> 15; x = x.wrapping_mul(0x31848bab);
    x ^= x >> 14; x
}
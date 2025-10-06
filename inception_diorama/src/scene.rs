use crate::math::{Vec3, lerp};
use crate::ray::Ray;
use crate::aabb::{Aabb, Hit};
use crate::material::{Material, Kind};

pub struct Scene {
    pub cubes: Vec<Aabb>,
    pub mats: Vec<Material>,
}

impl Scene {
    pub fn test_scene() -> Self {
        let mut mats = crate::material::default_materials();
        
        // Materiales base
        let grass_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.15, 0.35, 0.12), 
            specular: 0.02, 
            transparency: 0.0, 
            reflectivity: 0.02 
        });
        
        let dirt_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.28, 0.20, 0.14), 
            specular: 0.03, 
            transparency: 0.0, 
            reflectivity: 0.01 
        });
        
        // Materiales de la casa
        let wall_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.92, 0.88, 0.82), 
            specular: 0.08, 
            transparency: 0.0, 
            reflectivity: 0.03 
        });
        
        let stone_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.55, 0.52, 0.48), 
            specular: 0.06, 
            transparency: 0.0, 
            reflectivity: 0.02 
        });
        
        let wood_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.45, 0.32, 0.22), 
            specular: 0.05, 
            transparency: 0.0, 
            reflectivity: 0.02 
        });
        
        let roof_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.42, 0.28, 0.20), 
            specular: 0.05, 
            transparency: 0.0, 
            reflectivity: 0.03 
        });
        
        let window_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.15, 0.18, 0.22), 
            specular: 0.85, 
            transparency: 0.0, 
            reflectivity: 0.65 
        });
        
        // Materiales de vegetación
        let tree_trunk_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.35, 0.25, 0.18), 
            specular: 0.04, 
            transparency: 0.0, 
            reflectivity: 0.01 
        });
        
        let foliage_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.18, 0.42, 0.15), 
            specular: 0.05, 
            transparency: 0.0, 
            reflectivity: 0.02 
        });
        
        let crop_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.22, 0.48, 0.20), 
            specular: 0.04, 
            transparency: 0.0, 
            reflectivity: 0.02 
        });
        
        // Material de agua con reflexión
        let water_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.08, 0.15, 0.25), 
            specular: 0.40, 
            transparency: 0.0, 
            reflectivity: 0.55 
        });
        
        let fence_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.38, 0.30, 0.24), 
            specular: 0.05, 
            transparency: 0.0, 
            reflectivity: 0.02 
        });
        
        let mut cubes: Vec<Aabb> = Vec::new();
        
        // ============ TERRENO BASE ============
        // Plataforma principal de césped
        cubes.push(Aabb{ 
            min: Vec3::new(-18.0, -2.0, -18.0), 
            max: Vec3::new(18.0, -1.95, 18.0), 
            mat_id: grass_id 
        });
        
        // ============ CASA PRINCIPAL ============
        let house_x = 0.0;
        let house_z = 4.0;
        let house_y = -1.9;
        
        // Base de la casa
        let hw = 4.0; // ancho
        let hd = 3.5; // profundidad
        let hh = 2.5; // altura de paredes
        
        // Piso
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw, house_y - 0.1, house_z - hd), 
            max: Vec3::new(house_x + hw, house_y, house_z + hd), 
            mat_id: wood_id 
        });
        
        // Paredes (con huecos para puertas y ventanas)
        let wt = 0.2; // grosor de pared
        
        // Pared frontal (con puerta)
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw, house_y, house_z + hd - wt), 
            max: Vec3::new(house_x - 0.8, house_y + hh, house_z + hd), 
            mat_id: wall_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + 0.8, house_y, house_z + hd - wt), 
            max: Vec3::new(house_x + hw, house_y + hh, house_z + hd), 
            mat_id: wall_id 
        });
        
        // Marco de puerta
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - 0.8, house_y, house_z + hd - 0.05), 
            max: Vec3::new(house_x + 0.8, house_y + 2.0, house_z + hd), 
            mat_id: wood_id 
        });
        
        // Ventanas frontales
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - 2.5, house_y + 1.0, house_z + hd - 0.05), 
            max: Vec3::new(house_x - 1.5, house_y + 1.8, house_z + hd), 
            mat_id: window_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + 1.5, house_y + 1.0, house_z + hd - 0.05), 
            max: Vec3::new(house_x + 2.5, house_y + 1.8, house_z + hd), 
            mat_id: window_id 
        });
        
        // Pared trasera
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw, house_y, house_z - hd), 
            max: Vec3::new(house_x + hw, house_y + hh, house_z - hd + wt), 
            mat_id: wall_id 
        });
        
        // Paredes laterales (con ventanas)
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw, house_y, house_z - hd), 
            max: Vec3::new(house_x - hw + wt, house_y + hh, house_z + hd), 
            mat_id: wall_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + hw - wt, house_y, house_z - hd), 
            max: Vec3::new(house_x + hw, house_y + hh, house_z + hd), 
            mat_id: wall_id 
        });
        
        // Ventanas laterales
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw, house_y + 1.0, house_z - 0.6), 
            max: Vec3::new(house_x - hw + 0.05, house_y + 1.8, house_z + 0.6), 
            mat_id: window_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + hw - 0.05, house_y + 1.0, house_z - 0.6), 
            max: Vec3::new(house_x + hw, house_y + 1.8, house_z + 0.6), 
            mat_id: window_id 
        });
        
        // Zócalo de piedra
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw - 0.05, house_y - 0.08, house_z - hd - 0.05), 
            max: Vec3::new(house_x + hw + 0.05, house_y + 0.5, house_z + hd + 0.05), 
            mat_id: stone_id 
        });
        
        // Techo a dos aguas
        let roof_y = house_y + hh;
        let roof_h = 1.2;
        let roof_over = 0.6;
        
        // Lado izquierdo del techo
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw - roof_over, roof_y, house_z - hd - roof_over), 
            max: Vec3::new(house_x, roof_y + roof_h, house_z + hd + roof_over), 
            mat_id: roof_id 
        });
        
        // Lado derecho del techo
        cubes.push(Aabb{ 
            min: Vec3::new(house_x, roof_y, house_z - hd - roof_over), 
            max: Vec3::new(house_x + hw + roof_over, roof_y + roof_h, house_z + hd + roof_over), 
            mat_id: roof_id 
        });
        
        // Chimenea
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + 2.0, roof_y + roof_h, house_z - 1.0), 
            max: Vec3::new(house_x + 2.8, roof_y + roof_h + 1.0, house_z - 0.2), 
            mat_id: stone_id 
        });
        
        // ============ ÁRBOLES ============
        // Función helper para crear un árbol
        let add_tree = |cubes: &mut Vec<Aabb>, x: f32, z: f32, trunk_h: f32, foliage_size: f32| {
            // Tronco
            cubes.push(Aabb{ 
                min: Vec3::new(x - 0.25, -1.95, z - 0.25), 
                max: Vec3::new(x + 0.25, -1.95 + trunk_h, z + 0.25), 
                mat_id: tree_trunk_id 
            });
            
            // Follaje (3 niveles)
            let f_y = -1.95 + trunk_h - 0.3;
            cubes.push(Aabb{ 
                min: Vec3::new(x - foliage_size, f_y, z - foliage_size), 
                max: Vec3::new(x + foliage_size, f_y + foliage_size * 1.2, z + foliage_size), 
                mat_id: foliage_id 
            });
            cubes.push(Aabb{ 
                min: Vec3::new(x - foliage_size * 0.7, f_y + foliage_size * 0.8, z - foliage_size * 0.7), 
                max: Vec3::new(x + foliage_size * 0.7, f_y + foliage_size * 1.8, z + foliage_size * 0.7), 
                mat_id: foliage_id 
            });
            cubes.push(Aabb{ 
                min: Vec3::new(x - foliage_size * 0.4, f_y + foliage_size * 1.5, z - foliage_size * 0.4), 
                max: Vec3::new(x + foliage_size * 0.4, f_y + foliage_size * 2.3, z + foliage_size * 0.4), 
                mat_id: foliage_id 
            });
        };
        
        // Plantar árboles alrededor
        add_tree(&mut cubes, -8.0, 10.0, 3.5, 1.8);
        add_tree(&mut cubes, -10.5, 6.0, 3.2, 1.6);
        add_tree(&mut cubes, 9.0, 9.0, 3.8, 2.0);
        add_tree(&mut cubes, 11.0, 4.0, 3.0, 1.5);
        add_tree(&mut cubes, -12.0, -8.0, 3.6, 1.9);
        add_tree(&mut cubes, 10.0, -10.0, 3.3, 1.7);
        
        // ============ HUERTO DE CULTIVOS ============
        let crop_x = -8.0;
        let crop_z = -4.0;
        
        // Tierra del huerto
        cubes.push(Aabb{ 
            min: Vec3::new(crop_x - 4.0, -1.95, crop_z - 3.0), 
            max: Vec3::new(crop_x + 4.0, -1.90, crop_z + 3.0), 
            mat_id: dirt_id 
        });
        
        // Surcos de cultivo (filas)
        for i in 0..7 {
            let row_z = crop_z - 2.5 + (i as f32) * 0.85;
            for j in 0..9 {
                let col_x = crop_x - 3.5 + (j as f32) * 0.9;
                cubes.push(Aabb{ 
                    min: Vec3::new(col_x - 0.15, -1.90, row_z - 0.15), 
                    max: Vec3::new(col_x + 0.15, -1.50, row_z + 0.15), 
                    mat_id: crop_id 
                });
            }
        }
        
        // ============ SEGUNDO CULTIVO ============
        let crop2_x = 9.0;
        let crop2_z = -3.0;
        
        cubes.push(Aabb{ 
            min: Vec3::new(crop2_x - 3.0, -1.95, crop2_z - 2.5), 
            max: Vec3::new(crop2_x + 3.0, -1.90, crop2_z + 2.5), 
            mat_id: dirt_id 
        });
        
        for i in 0..6 {
            let row_z = crop2_z - 2.0 + (i as f32) * 0.8;
            for j in 0..7 {
                let col_x = crop2_x - 2.5 + (j as f32) * 0.85;
                cubes.push(Aabb{ 
                    min: Vec3::new(col_x - 0.15, -1.90, row_z - 0.15), 
                    max: Vec3::new(col_x + 0.15, -1.55, row_z + 0.15), 
                    mat_id: crop_id 
                });
            }
        }
        
        // ============ LAGUNA ============
        let pond_x = 5.0;
        let pond_z = 11.0;
        let pond_w = 5.0;
        let pond_d = 4.0;
        
        // Borde de tierra/piedra
        cubes.push(Aabb{ 
            min: Vec3::new(pond_x - pond_w - 0.3, -1.96, pond_z - pond_d - 0.3), 
            max: Vec3::new(pond_x + pond_w + 0.3, -1.88, pond_z + pond_d + 0.3), 
            mat_id: stone_id 
        });
        
        // Agua (ligeramente hundida)
        cubes.push(Aabb{ 
            min: Vec3::new(pond_x - pond_w, -1.93, pond_z - pond_d), 
            max: Vec3::new(pond_x + pond_w, -1.88, pond_z + pond_d), 
            mat_id: water_id 
        });
        
        // ============ CERCA DECORATIVA ============
        let add_fence_post = |cubes: &mut Vec<Aabb>, x: f32, z: f32| {
            cubes.push(Aabb{ 
                min: Vec3::new(x - 0.08, -1.95, z - 0.08), 
                max: Vec3::new(x + 0.08, -0.80, z + 0.08), 
                mat_id: fence_id 
            });
        };
        
        let add_fence_rail = |cubes: &mut Vec<Aabb>, x1: f32, z1: f32, x2: f32, z2: f32, y: f32| {
            cubes.push(Aabb{ 
                min: Vec3::new(x1.min(x2), y - 0.04, z1.min(z2) - 0.04), 
                max: Vec3::new(x1.max(x2), y + 0.04, z1.max(z2) + 0.04), 
                mat_id: fence_id 
            });
        };
        
        // Cerca alrededor del huerto 1
        for i in 0..9 {
            let fx = crop_x - 4.5 + (i as f32) * 1.1;
            add_fence_post(&mut cubes, fx, crop_z - 3.5);
            add_fence_post(&mut cubes, fx, crop_z + 3.5);
        }
        for i in 0..7 {
            let fz = crop_z - 3.0 + (i as f32) * 1.0;
            add_fence_post(&mut cubes, crop_x - 4.5, fz);
            add_fence_post(&mut cubes, crop_x + 4.5, fz);
        }
        
        // Rieles horizontales
        for i in 0..8 {
            let fx = crop_x - 4.5 + (i as f32) * 1.1;
            add_fence_rail(&mut cubes, fx, crop_z - 3.5, fx + 1.1, crop_z - 3.5, -1.2);
            add_fence_rail(&mut cubes, fx, crop_z + 3.5, fx + 1.1, crop_z + 3.5, -1.2);
        }
        
        // ============ CAMINO DE PIEDRA ============
        for i in 0..15 {
            let step_z = house_z + hd + 0.5 + (i as f32) * 0.6;
            let step_x = house_x + (i as f32) * 0.1 - 0.7;
            cubes.push(Aabb{ 
                min: Vec3::new(step_x - 0.4, -1.93, step_z - 0.35), 
                max: Vec3::new(step_x + 0.4, -1.91, step_z + 0.35), 
                mat_id: stone_id 
            });
        }
        
        // ============ DECORACIONES EXTRA ============
        // Barril
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + hw + 1.0, -1.90, house_z + hd - 1.5), 
            max: Vec3::new(house_x + hw + 1.6, -1.20, house_z + hd - 0.9), 
            mat_id: wood_id 
        });
        
        // Caja de herramientas
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw - 1.5, -1.90, house_z + hd - 1.0), 
            max: Vec3::new(house_x - hw - 0.7, -1.50, house_z + hd - 0.4), 
            mat_id: wood_id 
        });
        
        Self { cubes, mats }
    }
    
    pub fn sky(&self, d: Vec3) -> Vec3 {
        // Cielo diurno con gradiente suave
        let t = (d.y * 0.5 + 0.5).clamp(0.0, 1.0);
        let horizon = Vec3::new(0.70, 0.80, 0.95);
        let zenith = Vec3::new(0.40, 0.60, 0.95);
        lerp(horizon, zenith, t.powf(0.7))
    }
    
    pub fn trace(&self, ray: &Ray, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        
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
                
                // Luz direccional (sol)
                let sun_dir = Vec3::new(0.4, -0.7, 0.3).norm();
                let sun_color = Vec3::new(1.0, 0.98, 0.95) * 1.8;
                
                // Calcular sombra
                let bias = 0.001;
                let shadow_ray = Ray { 
                    o: h.p + h.n * bias, 
                    d: -sun_dir 
                };
                
                let mut in_shadow = false;
                for c in &self.cubes {
                    if c.hit(&shadow_ray, 0.001, 1000.0).is_some() {
                        in_shadow = true;
                        break;
                    }
                }
                
                // Iluminación difusa
                let ndotl = h.n.dot(-sun_dir).max(0.0);
                let shadow_factor = if in_shadow { 0.25 } else { 1.0 };
                let diffuse = mat.albedo * sun_color * ndotl * shadow_factor;
                
                // Luz ambiental
                let ambient = mat.albedo * Vec3::new(0.35, 0.40, 0.50) * 0.4;
                
                // Reflexión especular
                let mut specular = Vec3::new(0.0, 0.0, 0.0);
                if mat.reflectivity > 0.01 && depth > 1 {
                    let reflect_dir = ray.d - h.n * 2.0 * ray.d.dot(h.n);
                    let reflect_ray = Ray { 
                        o: h.p + h.n * bias, 
                        d: reflect_dir.norm() 
                    };
                    let reflect_color = self.trace(&reflect_ray, depth - 1);
                    specular = reflect_color * mat.reflectivity;
                }
                
                // Brillo especular (highlight)
                let view_dir = -ray.d;
                let reflect_dir = sun_dir - h.n * 2.0 * sun_dir.dot(h.n);
                let spec_factor = view_dir.dot(reflect_dir).max(0.0).powf(32.0);
                let highlight = Vec3::new(1.0, 1.0, 1.0) * spec_factor * mat.specular * shadow_factor;
                
                (ambient + diffuse + specular + highlight).clamp01()
            }
        }
    }
}

// Extensión para clamp a [0,1]
trait Clamp01 {
    fn clamp01(self) -> Self;
}

impl Clamp01 for Vec3 {
    fn clamp01(self) -> Self {
        Vec3::new(
            self.x.clamp(0.0, 1.0),
            self.y.clamp(0.0, 1.0),
            self.z.clamp(0.0, 1.0)
        )
    }
}
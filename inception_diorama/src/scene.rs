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
            albedo: Vec3::new(0.08, 0.12, 0.18), 
            specular: 0.95, 
            transparency: 0.0, 
            reflectivity: 0.85 
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
        
        // Material de agua para piscina (más cristalina)
        let pool_water_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.15, 0.35, 0.55), 
            specular: 0.80, 
            transparency: 0.0, 
            reflectivity: 0.70 
        });
        
        // Material de azulejo/baldosa para piscina
        let tile_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.70, 0.85, 0.95), 
            specular: 0.40, 
            transparency: 0.0, 
            reflectivity: 0.25 
        });
        
        let fence_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.38, 0.30, 0.24), 
            specular: 0.05, 
            transparency: 0.0, 
            reflectivity: 0.02 
        });
        
        // Material para cerdos (rosa Minecraft más brillante)
        let pig_body_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(1.0, 0.75, 0.80), 
            specular: 0.10, 
            transparency: 0.0, 
            reflectivity: 0.03 
        });
        
        let pig_snout_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.90, 0.55, 0.65), 
            specular: 0.08, 
            transparency: 0.0, 
            reflectivity: 0.02 
        });
        
        // Materiales para muebles
        let dark_wood_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.25, 0.18, 0.12), 
            specular: 0.15, 
            transparency: 0.0, 
            reflectivity: 0.05 
        });
        
        let cushion_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.65, 0.25, 0.20), 
            specular: 0.10, 
            transparency: 0.0, 
            reflectivity: 0.03 
        });
        
        let table_top_id = mats.len();
        mats.push(Material{ 
            kind: Kind::Diffuse, 
            albedo: Vec3::new(0.55, 0.40, 0.28), 
            specular: 0.25, 
            transparency: 0.0, 
            reflectivity: 0.08 
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
        
        // Ventanas frontales (más grandes y realistas)
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - 3.5, house_y + 0.9, house_z + hd - 0.05), 
            max: Vec3::new(house_x - 1.8, house_y + 2.0, house_z + hd + 0.01), 
            mat_id: window_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + 1.8, house_y + 0.9, house_z + hd - 0.05), 
            max: Vec3::new(house_x + 3.5, house_y + 2.0, house_z + hd + 0.01), 
            mat_id: window_id 
        });
        
        // Ventanas pequeñas superiores frontales (sobre la puerta)
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - 0.6, house_y + 2.1, house_z + hd - 0.05), 
            max: Vec3::new(house_x + 0.6, house_y + 2.4, house_z + hd + 0.01), 
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
        
        // Ventanas laterales (más grandes y múltiples)
        // Lado izquierdo - 2 ventanas
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw - 0.01, house_y + 0.9, house_z - 2.2), 
            max: Vec3::new(house_x - hw + 0.05, house_y + 2.0, house_z - 0.5), 
            mat_id: window_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw - 0.01, house_y + 0.9, house_z + 0.5), 
            max: Vec3::new(house_x - hw + 0.05, house_y + 2.0, house_z + 2.2), 
            mat_id: window_id 
        });
        
        // Lado derecho - 2 ventanas
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + hw - 0.05, house_y + 0.9, house_z - 2.2), 
            max: Vec3::new(house_x + hw + 0.01, house_y + 2.0, house_z - 0.5), 
            mat_id: window_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + hw - 0.05, house_y + 0.9, house_z + 0.5), 
            max: Vec3::new(house_x + hw + 0.01, house_y + 2.0, house_z + 2.2), 
            mat_id: window_id 
        });
        
        // Ventanas laterales pequeñas superiores
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw - 0.01, house_y + 2.1, house_z - 0.5), 
            max: Vec3::new(house_x - hw + 0.05, house_y + 2.4, house_z + 0.5), 
            mat_id: window_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + hw - 0.05, house_y + 2.1, house_z - 0.5), 
            max: Vec3::new(house_x + hw + 0.01, house_y + 2.4, house_z + 0.5), 
            mat_id: window_id 
        });
        
        // Ventanas traseras (3 ventanas)
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - 3.0, house_y + 1.0, house_z - hd - 0.01), 
            max: Vec3::new(house_x - 1.5, house_y + 1.9, house_z - hd + 0.05), 
            mat_id: window_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - 0.7, house_y + 1.0, house_z - hd - 0.01), 
            max: Vec3::new(house_x + 0.7, house_y + 1.9, house_z - hd + 0.05), 
            mat_id: window_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + 1.5, house_y + 1.0, house_z - hd - 0.01), 
            max: Vec3::new(house_x + 3.0, house_y + 1.9, house_z - hd + 0.05), 
            mat_id: window_id 
        });
        
        // Ventana trasera superior (buhardilla)
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - 0.6, house_y + 2.1, house_z - hd - 0.01), 
            max: Vec3::new(house_x + 0.6, house_y + 2.4, house_z - hd + 0.05), 
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
        
        // Ventanas en el techo (buhardillas/claraboyas)
        // Buhardilla frontal izquierda
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - 2.5, roof_y + 0.3, house_z + hd - 0.6), 
            max: Vec3::new(house_x - 1.5, roof_y + 0.7, house_z + hd - 0.55), 
            mat_id: window_id 
        });
        
        // Buhardilla frontal derecha
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + 1.5, roof_y + 0.3, house_z + hd - 0.6), 
            max: Vec3::new(house_x + 2.5, roof_y + 0.7, house_z + hd - 0.55), 
            mat_id: window_id 
        });
        
        // Claraboya lateral izquierda
        cubes.push(Aabb{ 
            min: Vec3::new(house_x - hw - roof_over + 0.05, roof_y + 0.4, house_z - 1.0), 
            max: Vec3::new(house_x - hw - roof_over + 0.1, roof_y + 0.8, house_z + 1.0), 
            mat_id: window_id 
        });
        
        // Claraboya lateral derecha
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + hw + roof_over - 0.1, roof_y + 0.4, house_z - 1.0), 
            max: Vec3::new(house_x + hw + roof_over - 0.05, roof_y + 0.8, house_z + 1.0), 
            mat_id: window_id 
        });
        
        // Chimenea
        cubes.push(Aabb{ 
            min: Vec3::new(house_x + 2.0, roof_y + roof_h, house_z - 1.0), 
            max: Vec3::new(house_x + 2.8, roof_y + roof_h + 1.0, house_z - 0.2), 
            mat_id: stone_id 
        });
        
        // ============ MUEBLES DENTRO DE LA CASA ============
        // Mesa de comedor (centro)
        let table_x = house_x;
        let table_z = house_z - 0.5;
        let table_y = house_y + 0.02;
        
        // Patas de la mesa (4)
        cubes.push(Aabb{ 
            min: Vec3::new(table_x - 0.9, table_y, table_z - 0.6), 
            max: Vec3::new(table_x - 0.75, table_y + 0.65, table_z - 0.45), 
            mat_id: dark_wood_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(table_x + 0.75, table_y, table_z - 0.6), 
            max: Vec3::new(table_x + 0.9, table_y + 0.65, table_z - 0.45), 
            mat_id: dark_wood_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(table_x - 0.9, table_y, table_z + 0.45), 
            max: Vec3::new(table_x - 0.75, table_y + 0.65, table_z + 0.6), 
            mat_id: dark_wood_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(table_x + 0.75, table_y, table_z + 0.45), 
            max: Vec3::new(table_x + 0.9, table_y + 0.65, table_z + 0.6), 
            mat_id: dark_wood_id 
        });
        
        // Superficie de la mesa
        cubes.push(Aabb{ 
            min: Vec3::new(table_x - 1.0, table_y + 0.65, table_z - 0.7), 
            max: Vec3::new(table_x + 1.0, table_y + 0.73, table_z + 0.7), 
            mat_id: table_top_id 
        });
        
        // Sillas alrededor de la mesa (4)
        let add_chair = |cubes: &mut Vec<Aabb>, cx: f32, cz: f32| {
            // Patas (4)
            let leg_h = 0.45;
            cubes.push(Aabb{ 
                min: Vec3::new(cx - 0.20, table_y, cz - 0.20), 
                max: Vec3::new(cx - 0.15, table_y + leg_h, cz - 0.15), 
                mat_id: dark_wood_id 
            });
            cubes.push(Aabb{ 
                min: Vec3::new(cx + 0.15, table_y, cz - 0.20), 
                max: Vec3::new(cx + 0.20, table_y + leg_h, cz - 0.15), 
                mat_id: dark_wood_id 
            });
            cubes.push(Aabb{ 
                min: Vec3::new(cx - 0.20, table_y, cz + 0.15), 
                max: Vec3::new(cx - 0.15, table_y + leg_h, cz + 0.20), 
                mat_id: dark_wood_id 
            });
            cubes.push(Aabb{ 
                min: Vec3::new(cx + 0.15, table_y, cz + 0.15), 
                max: Vec3::new(cx + 0.20, table_y + leg_h, cz + 0.20), 
                mat_id: dark_wood_id 
            });
            // Asiento
            cubes.push(Aabb{ 
                min: Vec3::new(cx - 0.22, table_y + leg_h, cz - 0.22), 
                max: Vec3::new(cx + 0.22, table_y + leg_h + 0.06, cz + 0.22), 
                mat_id: cushion_id 
            });
            // Respaldo
            cubes.push(Aabb{ 
                min: Vec3::new(cx - 0.22, table_y + leg_h, cz - 0.22), 
                max: Vec3::new(cx + 0.22, table_y + leg_h + 0.45, cz - 0.18), 
                mat_id: dark_wood_id 
            });
        };
        
        add_chair(&mut cubes, table_x - 1.3, table_z);
        add_chair(&mut cubes, table_x + 1.3, table_z);
        add_chair(&mut cubes, table_x, table_z - 1.0);
        add_chair(&mut cubes, table_x, table_z + 1.0);
        
        // Sofá (esquina izquierda)
        let sofa_x = house_x - 2.5;
        let sofa_z = house_z + 2.0;
        
        // Base del sofá
        cubes.push(Aabb{ 
            min: Vec3::new(sofa_x - 0.8, table_y, sofa_z - 0.4), 
            max: Vec3::new(sofa_x + 0.8, table_y + 0.35, sofa_z + 0.4), 
            mat_id: cushion_id 
        });
        
        // Respaldo
        cubes.push(Aabb{ 
            min: Vec3::new(sofa_x - 0.8, table_y + 0.35, sofa_z - 0.4), 
            max: Vec3::new(sofa_x + 0.8, table_y + 0.75, sofa_z - 0.3), 
            mat_id: cushion_id 
        });
        
        // Brazos
        cubes.push(Aabb{ 
            min: Vec3::new(sofa_x - 0.8, table_y + 0.35, sofa_z - 0.4), 
            max: Vec3::new(sofa_x - 0.65, table_y + 0.55, sofa_z + 0.4), 
            mat_id: dark_wood_id 
        });
        cubes.push(Aabb{ 
            min: Vec3::new(sofa_x + 0.65, table_y + 0.35, sofa_z - 0.4), 
            max: Vec3::new(sofa_x + 0.8, table_y + 0.55, sofa_z + 0.4), 
            mat_id: dark_wood_id 
        });
        
        // Estantería (pared trasera)
        let shelf_x = house_x + 2.5;
        let shelf_z = house_z - 2.8;
        
        cubes.push(Aabb{ 
            min: Vec3::new(shelf_x - 0.6, table_y, shelf_z - 0.15), 
            max: Vec3::new(shelf_x + 0.6, table_y + 1.5, shelf_z), 
            mat_id: dark_wood_id 
        });
        
        // Repisas (3 niveles)
        for i in 0..3 {
            let shelf_h = table_y + 0.4 + (i as f32) * 0.35;
            cubes.push(Aabb{ 
                min: Vec3::new(shelf_x - 0.55, shelf_h, shelf_z - 0.12), 
                max: Vec3::new(shelf_x + 0.55, shelf_h + 0.04, shelf_z - 0.02), 
                mat_id: table_top_id 
            });
        }
        
        // Cama (esquina derecha trasera)
        let bed_x = house_x + 2.2;
        let bed_z = house_z + 1.5;
        
        // Marco de la cama
        cubes.push(Aabb{ 
            min: Vec3::new(bed_x - 0.65, table_y, bed_z - 1.0), 
            max: Vec3::new(bed_x + 0.65, table_y + 0.25, bed_z + 1.0), 
            mat_id: dark_wood_id 
        });
        
        // Colchón
        cubes.push(Aabb{ 
            min: Vec3::new(bed_x - 0.6, table_y + 0.25, bed_z - 0.95), 
            max: Vec3::new(bed_x + 0.6, table_y + 0.40, bed_z + 0.95), 
            mat_id: cushion_id 
        });
        
        // Cabecera
        cubes.push(Aabb{ 
            min: Vec3::new(bed_x - 0.65, table_y, bed_z - 1.0), 
            max: Vec3::new(bed_x + 0.65, table_y + 0.70, bed_z - 0.92), 
            mat_id: dark_wood_id 
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

        // ============ CERDO (estilo Minecraft) ============
        let add_pig = |cubes: &mut Vec<Aabb>, x: f32, z: f32, s: f32| {
            // Altura del suelo
            let gy0 = -1.95;
            // Dimensiones base escaladas
            let body_l = 1.20 * s; // largo (eje Z)
            let body_w = 0.80 * s; // ancho (eje X)
            let body_h = 0.65 * s; // alto

            let leg_w  = 0.18 * s;
            let leg_h  = 0.32 * s;

            let head_l = 0.45 * s;
            let head_w = 0.55 * s;
            let head_h = 0.50 * s;

            let snout_l = 0.18 * s;
            let snout_w = 0.40 * s;
            let snout_h = 0.28 * s;

            // Cuerpo (centrado en x, z)
            let body_y0 = gy0 + leg_h; // sobre las patas
            let body_y1 = body_y0 + body_h;
            cubes.push(Aabb{
                min: Vec3::new(x - body_w*0.5, body_y0, z - body_l*0.5),
                max: Vec3::new(x + body_w*0.5, body_y1, z + body_l*0.5),
                mat_id: pig_body_id
            });

            // Cabeza (al frente en +Z)
            let head_y0 = body_y0 + 0.05 * s;
            let head_y1 = head_y0 + head_h;
            let head_z0 = z + body_l*0.5; // arranca desde el frente del cuerpo
            cubes.push(Aabb{
                min: Vec3::new(x - head_w*0.5, head_y0, head_z0),
                max: Vec3::new(x + head_w*0.5, head_y1, head_z0 + head_l),
                mat_id: pig_body_id
            });

            // Hocico (snout)
            let snout_y0 = head_y0 + 0.12 * s;
            let snout_y1 = snout_y0 + snout_h;
            let snout_z0 = head_z0 + head_l - 0.02 * s;
            cubes.push(Aabb{
                min: Vec3::new(x - snout_w*0.5, snout_y0, snout_z0),
                max: Vec3::new(x + snout_w*0.5, snout_y1, snout_z0 + snout_l),
                mat_id: pig_snout_id
            });

            // Orejas (dos cubitos arriba de la cabeza)
            let ear_w = 0.14 * s; let ear_h = 0.16 * s; let ear_l = 0.10 * s;
            let ear_y0 = head_y1 - ear_h * 0.6;
            let ear_z0 = head_z0 + head_l * 0.25;
            // Izquierda
            cubes.push(Aabb{
                min: Vec3::new(x - head_w*0.5 + 0.06 * s, ear_y0, ear_z0),
                max: Vec3::new(x - head_w*0.5 + 0.06 * s + ear_w, ear_y0 + ear_h, ear_z0 + ear_l),
                mat_id: pig_body_id
            });
            // Derecha
            cubes.push(Aabb{
                min: Vec3::new(x + head_w*0.5 - 0.06 * s - ear_w, ear_y0, ear_z0),
                max: Vec3::new(x + head_w*0.5 - 0.06 * s, ear_y0 + ear_h, ear_z0 + ear_l),
                mat_id: pig_body_id
            });

            // Patas (4)
            let lp_y0 = gy0; let lp_y1 = gy0 + leg_h;
            let off_x = body_w*0.5 - leg_w*0.6;
            let off_z = body_l*0.5 - leg_w*0.6;
            // Delantera izquierda (x-, z+)
            cubes.push(Aabb{ min: Vec3::new(x - off_x - leg_w, lp_y0, z + off_z - leg_w), max: Vec3::new(x - off_x, lp_y1, z + off_z), mat_id: pig_body_id });
            // Delantera derecha (x+, z+)
            cubes.push(Aabb{ min: Vec3::new(x + off_x, lp_y0, z + off_z - leg_w), max: Vec3::new(x + off_x + leg_w, lp_y1, z + off_z), mat_id: pig_body_id });
            // Trasera izquierda (x-, z-)
            cubes.push(Aabb{ min: Vec3::new(x - off_x - leg_w, lp_y0, z - off_z), max: Vec3::new(x - off_x, lp_y1, z - off_z + leg_w), mat_id: pig_body_id });
            // Trasera derecha (x+, z-)
            cubes.push(Aabb{ min: Vec3::new(x + off_x, lp_y0, z - off_z), max: Vec3::new(x + off_x + leg_w, lp_y1, z - off_z + leg_w), mat_id: pig_body_id });

            // Colita (pequeño cubo atrás)
            let tail_w = 0.08 * s; let tail_h = 0.10 * s; let tail_l = 0.06 * s;
            cubes.push(Aabb{
                min: Vec3::new(x - tail_w*0.5, body_y0 + body_h*0.6, z - body_l*0.5 - tail_l),
                max: Vec3::new(x + tail_w*0.5, body_y0 + body_h*0.6 + tail_h, z - body_l*0.5),
                mat_id: pig_body_id
            });
        };

        // Un solo cerdo, al noreste de la casa, lejos de la piscina
        add_pig(&mut cubes, house_x - 5.8, house_z + 6.8, 1.0);
        
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
        
        // ============ PISCINA REALISTA ============
        let pool_x = 0.0;
        let pool_z = 11.5;
        let pool_w = 4.5; // ancho
        let pool_d = 3.0; // profundidad
        let pool_depth = 0.35; // profundidad del agua
        
        // Deck de baldosas alrededor de la piscina
        let deck_size = 1.0;
        cubes.push(Aabb{ 
            min: Vec3::new(pool_x - pool_w - deck_size, -1.93, pool_z - pool_d - deck_size), 
            max: Vec3::new(pool_x + pool_w + deck_size, -1.90, pool_z + pool_d + deck_size), 
            mat_id: tile_id 
        });
        
        // Pared interior de la piscina (azulejos claros)
        let pool_bottom = -1.90 - pool_depth;
        
        // Piso de la piscina
        cubes.push(Aabb{ 
            min: Vec3::new(pool_x - pool_w, pool_bottom, pool_z - pool_d), 
            max: Vec3::new(pool_x + pool_w, pool_bottom + 0.03, pool_z + pool_d), 
            mat_id: tile_id 
        });
        
        // Paredes interiores de azulejos
        let wall_thick = 0.08;
        
        // Pared Norte
        cubes.push(Aabb{ 
            min: Vec3::new(pool_x - pool_w, pool_bottom, pool_z + pool_d - wall_thick), 
            max: Vec3::new(pool_x + pool_w, -1.90, pool_z + pool_d), 
            mat_id: tile_id 
        });
        
        // Pared Sur
        cubes.push(Aabb{ 
            min: Vec3::new(pool_x - pool_w, pool_bottom, pool_z - pool_d), 
            max: Vec3::new(pool_x + pool_w, -1.90, pool_z - pool_d + wall_thick), 
            mat_id: tile_id 
        });
        
        // Pared Oeste
        cubes.push(Aabb{ 
            min: Vec3::new(pool_x - pool_w, pool_bottom, pool_z - pool_d), 
            max: Vec3::new(pool_x - pool_w + wall_thick, -1.90, pool_z + pool_d), 
            mat_id: tile_id 
        });
        
        // Pared Este
        cubes.push(Aabb{ 
            min: Vec3::new(pool_x + pool_w - wall_thick, pool_bottom, pool_z - pool_d), 
            max: Vec3::new(pool_x + pool_w, -1.90, pool_z + pool_d), 
            mat_id: tile_id 
        });
        
        // Agua de la piscina (cristalina y reflectante)
        cubes.push(Aabb{ 
            min: Vec3::new(pool_x - pool_w + wall_thick, -1.90 - pool_depth + 0.03, pool_z - pool_d + wall_thick), 
            max: Vec3::new(pool_x + pool_w - wall_thick, -1.885, pool_z + pool_d - wall_thick), 
            mat_id: pool_water_id 
        });
        
        // Escalera de la piscina (3 escalones)
        let stair_x = pool_x + pool_w - wall_thick - 0.8;
        for i in 0..3 {
            let step_y = -1.90 - (i as f32 + 1.0) * (pool_depth / 3.5);
            let step_depth = 0.3 + i as f32 * 0.1;
            cubes.push(Aabb{ 
                min: Vec3::new(stair_x, step_y, pool_z - 0.6), 
                max: Vec3::new(stair_x + step_depth, step_y + 0.08, pool_z + 0.6), 
                mat_id: tile_id 
            });
        }
        
        // Tumbonas junto a la piscina
        let add_lounger = |cubes: &mut Vec<Aabb>, x: f32, z: f32| {
            // Base
            cubes.push(Aabb{ 
                min: Vec3::new(x - 0.4, -1.89, z - 1.0), 
                max: Vec3::new(x + 0.4, -1.75, z + 1.0), 
                mat_id: wood_id 
            });
            // Respaldo
            cubes.push(Aabb{ 
                min: Vec3::new(x - 0.4, -1.75, z - 1.0), 
                max: Vec3::new(x + 0.4, -1.35, z - 0.85), 
                mat_id: wood_id 
            });
        };
        
        add_lounger(&mut cubes, pool_x - pool_w - deck_size - 0.8, pool_z - 1.5);
        add_lounger(&mut cubes, pool_x - pool_w - deck_size - 0.8, pool_z + 1.5);
        add_lounger(&mut cubes, pool_x + pool_w + deck_size + 0.8, pool_z);
        
        // ============ CERCA DECORATIVA =======
        
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
                    
                    // Para ventanas: mezclar reflexión con un tinte de vidrio
                    if mat.reflectivity > 0.7 {
                        // Ventanas: mezcla de reflexión pura con tinte azulado
                        let glass_tint = Vec3::new(0.85, 0.90, 1.0);
                        specular = reflect_color * glass_tint * mat.reflectivity;
                    } else {
                        // Otros materiales reflectantes
                        specular = reflect_color * mat.reflectivity;
                    }
                }
                
                // Brillo especular (highlight)
                let view_dir = -ray.d;
                let reflect_dir = sun_dir - h.n * 2.0 * sun_dir.dot(h.n);
                let spec_factor = view_dir.dot(reflect_dir).max(0.0).powf(64.0);
                let highlight = Vec3::new(1.0, 1.0, 1.0) * spec_factor * mat.specular * shadow_factor * 0.8;
                
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
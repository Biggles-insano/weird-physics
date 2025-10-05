// src/main.rs
mod math;     mod ray;     mod camera;
mod aabb;     mod material; mod scene;

use minifb::{Key, Window, WindowOptions};
use math::Vec3;
use camera::Camera;

fn main() {
    let mut w: usize = 960;
    let mut h: usize = 540;
    let mut window = Window::new(
        "Diorama — Flechas: yaw/pitch | Z/X: roll | Q/E: dolly",
        w, h, WindowOptions { resize: true, scale: minifb::Scale::X1, ..WindowOptions::default() }
    ).unwrap();

    let mut fb = vec![0u32; w*h];

    // Cámara que gira en su eje
    let mut eye = Vec3::new(0.0, 0.0, 12.0);
    let mut yaw:f32 = 0.0;
    let mut pitch:f32 = 0.0;
    let mut roll:f32 = 0.0;
    let rot_step = 1.5;
    let dolly = 0.35;

    let scene = scene::Scene::test_scene();
    let mut fov = 60.0_f32;

    while window.is_open() {
        let (nw, nh) = window.get_size();
        if nw != w || nh != h {
            w = nw.max(1);
            h = nh.max(1);
            fb.resize(w * h, 0);
        }   
        // Controles
        if window.is_key_down(Key::Left)  { yaw -= rot_step; }
        if window.is_key_down(Key::Right) { yaw += rot_step; }
        if window.is_key_down(Key::Up)    { pitch = (pitch + rot_step).clamp(-85.0,85.0); }
        if window.is_key_down(Key::Down)  { pitch = (pitch - rot_step).clamp(-85.0,85.0); }
        if window.is_key_down(Key::Z)     { roll -= rot_step; }
        if window.is_key_down(Key::X)     { roll += rot_step; }

        let cam = Camera::from_euler(eye, yaw, pitch, roll, fov, w as f32 / h as f32);

        // Dolly sobre su eje forward
        let forward = (-cam.w).norm();
        if window.is_key_down(Key::Q) { eye = eye + forward * (-dolly); }
        if window.is_key_down(Key::E) { eye = eye + forward * (dolly);  }

        // Render
        let inv_w = 1.0 / w as f32;
        let inv_h = 1.0 / h as f32;
        for j in 0..h {
            let y = ((j as f32)*inv_h)*2.0 - 1.0; // [-1,1]
            for i in 0..w {
                let x = ((i as f32)*inv_w)*2.0 - 1.0; // [-1,1]
                let ray = cam.ray_for(x, -y); // y invertida para imagen
                let col = scene.trace(&ray, 2).clamp01(); // 1 rebote máx
                fb[j*w+i] = rgb_u32(col.x, col.y, col.z);
            }
        }

        window.update_with_buffer(&fb, w, h).unwrap();
    }
}

fn rgb_u32(r:f32,g:f32,b:f32)->u32{
    let (r,g,b)=((r*255.0) as u32, (g*255.0) as u32, (b*255.0) as u32);
    (255<<24) | (r<<16) | (g<<8) | b
}
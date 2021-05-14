use std::fs;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::random;

fn hue_to_rgb(p: f64, q: f64, t:f64) -> f64 {
    let mut t = t;
    if t < 0.0 { t += 1.0 }
    if t > 1.0 { t -= 1.0 }
    if t < 1.0/6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0/2.0 {
        q
    } else if t < 2.0/3.0 {
        p + (q - p) * (2.0/3.0 - t) * 6.0
    } else {
        p
    }
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    if s == 0.0 {
        (l, l, l)
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        (hue_to_rgb(p, q, h + 1.0/3.0), hue_to_rgb(p, q, h), hue_to_rgb(p, q, h - 1.0/3.0))
    }
}

fn f64_color_to_u8(rgb: (f64, f64, f64)) -> (u8, u8, u8) {
    let (r, g, b) = rgb;
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn export_as_ppm(framebuffer: &Vec<u32>, width: usize, height: usize, file_name: &str) {
    let mut to_write = format!("P3\n{} {}\n255\n", width, height);
    for pixel in framebuffer {
        let color = pixel.to_le_bytes();
        to_write.push_str(&format!("{} {} {}\n", color[2], color[1], color[0]));
    }
    fs::write(file_name, to_write).expect(&format!("Failed to write: {}", file_name));
}

fn main() {
    const SCREEN_WIDTH: usize = 128;
    const SCREEN_HEIGHT: usize = 128;
    const PALETTE_SIZE: usize = 256;

    let mut fire_buf: Vec<usize> = vec![0; SCREEN_HEIGHT * SCREEN_WIDTH];
    let mut frame_buf: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut palette: Vec<u32> = vec![0; PALETTE_SIZE];

    let mut window = Window::new("Classic Fire Effect", SCREEN_WIDTH, SCREEN_HEIGHT, WindowOptions::default()).unwrap();
    let mut current_hue = 3.0;
    for entry_index in 0..palette.len() {
        let (r, g, b) = f64_color_to_u8(hsl_to_rgb(entry_index as f64 /(current_hue * palette.len() as f64), 1.0, ((entry_index as f64 * 2.0) / palette.len() as f64).min(1.0)));
        palette[entry_index] = u32::from_le_bytes([b, g, r, 0x00]);
    }

    while window.is_open() {
        for x in 0..SCREEN_WIDTH {
            fire_buf[(SCREEN_HEIGHT - 1) * SCREEN_WIDTH + x] = (32768 + random::<usize>()) % 256;
        }
        for y in 0..(SCREEN_HEIGHT) {
            for x in 0..SCREEN_WIDTH {
                fire_buf[y * SCREEN_WIDTH + x] =
                    ((fire_buf[((y + 1) % SCREEN_HEIGHT) * SCREEN_WIDTH + ((x - 1 + SCREEN_WIDTH) % SCREEN_WIDTH)]
                    + fire_buf[((y + 1) % SCREEN_HEIGHT) * SCREEN_WIDTH + (x % SCREEN_WIDTH)]
                    + fire_buf[((y + 1) % SCREEN_HEIGHT) * SCREEN_WIDTH + ((x + 1) % SCREEN_WIDTH)]
                    + fire_buf[((y + 2) % SCREEN_HEIGHT) * SCREEN_WIDTH + (x % SCREEN_WIDTH)])
                    * 32) / 129;
            }

            for index in 0..frame_buf.len() {
                frame_buf[index] = palette[fire_buf[index]];
            }

            if window.is_key_down(Key::F12) {
                export_as_ppm(&frame_buf, SCREEN_WIDTH, SCREEN_HEIGHT, "shot.ppm");
            }

            if window.is_key_pressed(Key::F11, KeyRepeat::Yes) {
                current_hue += 0.05;
                println!("{}", current_hue);
                for entry_index in 0..palette.len() {
                    let (r, g, b) = f64_color_to_u8(hsl_to_rgb(entry_index as f64 /(current_hue * palette.len() as f64), 1.0, ((entry_index as f64 * 2.0) / palette.len() as f64).min(1.0)));
                    palette[entry_index] = u32::from_le_bytes([b, g, r, 0x00]);
                }
            }

            if window.is_key_pressed(Key::F10, KeyRepeat::Yes) {
                current_hue -= 0.05;
                println!("{}", current_hue);
                for entry_index in 0..palette.len() {
                    let (r, g, b) = f64_color_to_u8(hsl_to_rgb(entry_index as f64 /(current_hue * palette.len() as f64), 1.0, ((entry_index as f64 * 2.0) / palette.len() as f64).min(1.0)));
                    palette[entry_index] = u32::from_le_bytes([b, g, r, 0x00]);
                }
            }

            window.update_with_buffer(&frame_buf, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
        }
    }
}

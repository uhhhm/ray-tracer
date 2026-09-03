mod vec3;
mod ray;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let width: i32 = 256;
    let height: i32 = 256;
    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{width} {height}").unwrap();
    writeln!(writer, "255").unwrap();
    let b = (0.25 * 255.0 as f64).round() as i32;
    for i in 0..height{
        for j in 0..width{
            let r = (i as f64 / (height - 1) as f64 * 255.0).round() as i32;
            let g = (j as f64 / (width - 1) as f64 * 255.0).round() as i32;


            writeln!(writer, "{r} {g} {b}").unwrap();
        }
    }
}

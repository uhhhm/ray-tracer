mod vec3;
mod ray;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::vec3::{Vec3, Point3, Color};
use crate::ray::Ray;

fn main() {
    let width: i32 = 256;
    let height: i32 = 256;
    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{width} {height}").unwrap();
    writeln!(writer, "255").unwrap();
    // let b = (0.25 * 255.0 as f64).round() as i32;
    // for i in 0..height{
    //     for j in 0..width{
    //         let r = (i as f64 / (height - 1) as f64 * 255.0).round() as i32;
    //         let g = (j as f64 / (width - 1) as f64 * 255.0).round() as i32;

    //         writeln!(writer, "{r} {g} {b}").unwrap();
    //     }
    // }

    let cam_center = Point3::new(0.0, 0.0, 0.0);
    let focal_length = 2.0;

    let vp_width = 2.0;
    let vp_height = 2.0;

    let vp_u = Vec3::new(vp_width, 0.0, 0.0);
    let vp_v = Vec3::new(0.0, -vp_height as f64, 0.0);

    let pixel_delta_u = vp_u / (width as f64);
    let pixel_delta_v = vp_v / (height as f64);

    let vp_upper_left = cam_center - Vec3::new(0.0, 0.0, focal_length) - vp_u/2.0 - vp_v/2.0;
    let upper_left_pixel = vp_upper_left + (pixel_delta_u + pixel_delta_v)/2.0;

    for i in 0..height{
        for j in 0..width{
            let pixel_center = upper_left_pixel + pixel_delta_u * (j as f64) + pixel_delta_v * (i as f64);

            let ray_direction = pixel_center - cam_center;
            let rd_normalised = ray_direction.normalised();

            let t = 0.5 * (rd_normalised.y + 1.0);
            let color = Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;

            let ray = Ray::new(cam_center, pixel_center-cam_center);
            writeln!(writer, "{} {} {}", ((color.x*255.0) as i32), ((color.y*255.0) as i32), ((color.z*255.0) as i32)).unwrap();
        }
    }
}

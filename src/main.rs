mod vec3;
mod ray;
mod hit_record;
mod hittable;
mod hittable_list;
mod sphere;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::vec3::{Vec3, Point3, Color};
use crate::ray::Ray;

const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;

fn ray_color(ray: &Ray) -> Color {
    let rd_normalised = ray.direction.normalised();
    let t = 0.5 * (rd_normalised.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn write_header(writer: &mut BufWriter<File>){
    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{WIDTH} {HEIGHT}").unwrap();
    writeln!(writer, "255").unwrap();
}

fn write_color(writer: &mut BufWriter<File>, color:Color){
    writeln!(writer, "{} {} {}", ((color.x*255.0) as i32), ((color.y*255.0) as i32), ((color.z*255.0) as i32)).unwrap();
}

fn main() {
    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    write_header(&mut writer);

    let cam_center = Point3::new(0.0, 0.0, 0.0);
    let focal_length = 2.0;

    let vp_width = 2.0;
    let vp_height = 2.0;

    let vp_u = Vec3::new(vp_width, 0.0, 0.0);
    let vp_v = Vec3::new(0.0, -vp_height as f64, 0.0);

    let pixel_delta_u = vp_u / (WIDTH as f64);
    let pixel_delta_v = vp_v / (HEIGHT as f64);

    let vp_upper_left = cam_center - Vec3::new(0.0, 0.0, focal_length) - vp_u/2.0 - vp_v/2.0;
    let upper_left_pixel = vp_upper_left + (pixel_delta_u + pixel_delta_v)/2.0;

    for i in 0..HEIGHT{
        for j in 0..WIDTH{
            let pixel_center = upper_left_pixel + pixel_delta_u * (j as f64) + pixel_delta_v * (i as f64);

            let ray_direction = pixel_center - cam_center;
            let ray = Ray::new(cam_center, ray_direction);

            let color = ray_color(&ray);
            write_color(&mut writer, color);
        }
    }
}

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use std::{io::Write, rc::Rc, time::Instant};

use camera::Camera;
use color::Color;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Metal};
use ray::Point3;
use rtweekend::{random_double, random_double_range};
use sphere::*;
use vec3::Vec3;

fn main() {
    let start_time = Instant::now();

    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(Color { e: [0.5, 0.5, 0.5] }));

    world.add(Rc::new(Sphere::new(
        Vec3 {
            e: [0.0, -1000.0, 0.0],
        },
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));

    world.add(Rc::new(Sphere::new(
        Point3 { e: [0.0, 1.0, 0.0] },
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color { e: [0.4, 0.2, 0.1] }));

    world.add(Rc::new(Sphere::new(
        Point3 {
            e: [-4.0, 1.0, 0.0],
        },
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color { e: [0.7, 0.6, 0.5] }, 0.0));

    world.add(Rc::new(Sphere::new(
        Vec3 { e: [4.0, 1.0, 0.0] },
        1.0,
        material3,
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let image = camera.render(&world);

    for p in image.chunks(3) {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();

        writeln!(lock, "{} {} {}", p[0], p[1], p[2]).unwrap();
    }

    eprintln!("Done!");

    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);

    eprintln!("Time: {:?}", elapsed_time);
}

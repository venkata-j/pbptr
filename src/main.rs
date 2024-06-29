// use std::io;
// use std::io::Write;
mod vec3;
mod ray;
mod hittable;
mod util;
mod camera;
mod material;
use vec3::*;
use hittable::*;
use util::*;
use std::{f64::INFINITY, rc::Rc};
use camera::*;
use rand::{thread_rng, Rng};
use material::*;
use std::sync::Arc;
use tobj::{self, LoadOptions};

fn main() {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Material::Lambertian(Lambertian { albedo: colour(0.5, 0.5, 0.5) }));

    let mat = Arc::new(
        Material::Metal(Metal{ albedo: colour(0.6, 0.6, 0.6), fuzz: 0.3 })
    );
    let mat1 = Arc::new(
        Material::Lambertian(Lambertian{ albedo: colour(0.4, 0.4, 0.7) })
    );
    let mat2 = Arc::new(
        Material::Dielectric(Dielectric{ mu: 1.5 })
    );
    let mat3 = Arc::new(Material::TestMaterial(TestMaterial{ albedo: colour(0,0,0) }));

    let load_options = LoadOptions { single_index: false, triangulate: true, ignore_points: true, ignore_lines: true };

    let scene = tobj::load_obj("suzanne.obj", &load_options);
    assert!(scene.is_ok());
    let (models, materials) = scene.expect("Failed to load OBJ file");


    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        assert_eq!(mesh.indices.len() % 3, 0);
        for v in 0..mesh.indices.len() / 3 {
            let i1 = mesh.indices[3*v] as usize;
            let i2 = mesh.indices[3*v + 1] as usize;
            let i3 = mesh.indices[3*v + 2] as usize;

            let v1 = point3(
                mesh.positions[3*i1], mesh.positions[3*i1 + 1], mesh.positions[3*i1 + 2]
            );
            let v2 = point3(
                mesh.positions[3*i2], mesh.positions[3*i2 + 1], mesh.positions[3*i2 + 2]
            );
            let v3 = point3(
                mesh.positions[3*i3], mesh.positions[3*i3 + 1], mesh.positions[3*i3 + 2]
            );

            let triangle = Triangle::new(v1, v2, v3, mat.clone());
            world.add(Arc::new(Hittable::Triangle(triangle)));
        }
    }

    world.add(Arc::new(Hittable::Sphere(Sphere::new(point3(-2,0,2),1.0,mat.clone()))));
    world.add(Arc::new(Hittable::Sphere(Sphere::new(point3(0,-102,0),100.0,ground_material))));

    let mut cam = Camera::new(16.0/9.0, 500);
    cam.sample_rate = 50;
    cam.max_ray_bounces = 25;
    cam.vfov = 20.0;
    cam.look_from = point3(5.0, 0, 5.0);
    cam.look_at = point3(0, 0, 0);
    cam.vup = vec3(0, 1, 0);

    cam.defocus_angle = 0.0;
    cam.focus_distance = 2.0;

    cam = cam.reinit();
    cam.render(&Hittable::HittableList(world));
}

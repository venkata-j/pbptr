use crate::vec3::*;
use crate::ray::*;
use crate::util::*;
use crate::hittable::*;
use crate::material::*;
use std::f64::INFINITY;
use std::fs::File;
use std::io::stdout;
use std::io::BufWriter;
use std::io::Write;
use std::time::Instant;
use rand::thread_rng;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00: Point3,
    delta_u: Point3,
    delta_v: Point3,
    pub max_ray_bounces: i32,
    pub sample_rate: i32,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3
}

impl Camera {
    pub fn new(ratio_: f64, width: i32) -> Self {
        let aspect_ratio = ratio_;
        let image_width = width;
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let look_from = point3(0, 0, 0);
        let look_at = point3(0, 0, -1);
        let vup = vec3(0, 1, 0);

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);

        // constants for viewport
        // let focal_length = (look_from - look_at).norm();
        let vfov = 90.0;
        let theta = deg_to_rad(vfov);
        let h = (0.5*theta).tan();
        let viewport_height = 2.0 * h * 10.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = look_from;

        let v_u = viewport_width * u;
        let v_v = -viewport_height * v;

        let delta_u = v_u / image_width as f64;
        let delta_v = v_v / image_height as f64;

        let viewport_upperleft = camera_center - 10.0*w - v_u/2.0 - v_v/2.0;
        let pixel00 = viewport_upperleft + 0.5 * (delta_u + delta_v);

        Self {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            center: camera_center,
            pixel00: pixel00,
            delta_u: delta_u,
            delta_v: delta_v,
            max_ray_bounces: 50,
            sample_rate: 40,
            vfov: 90.0,
            look_from: look_from,
            look_at: look_at,
            vup: vup,
            defocus_angle: 0.0,
            focus_distance: 3.4,
            defocus_disc_u: vec3(0,0,0),
            defocus_disc_v: vec3(0,0,0)
        }
    }

    pub fn reinit(&self) -> Self {
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let look_from = self.look_from;
        let look_at = self.look_at;
        let vup = self.vup;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);

        // constants for viewport
        // let focal_length = (look_from - look_at).norm();
        let theta = deg_to_rad(self.vfov);
        let h = (0.5*theta).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = look_from;

        let v_u = viewport_width * u;
        let v_v = -viewport_height * v;

        let delta_u = v_u / image_width as f64;
        let delta_v = v_v / image_height as f64;

        let viewport_upperleft = camera_center - (self.focus_distance*w) - v_u/2.0 - v_v/2.0;
        let pixel00 = viewport_upperleft + 0.5 * (delta_u + delta_v);

        let defocus_radius = self.focus_distance * ((deg_to_rad(self.defocus_angle/2.0)).tan());
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        Self {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            center: camera_center,
            pixel00: pixel00,
            delta_u: delta_u,
            delta_v: delta_v,
            max_ray_bounces: self.max_ray_bounces,
            sample_rate: self.sample_rate,
            vfov: self.vfov,
            look_from: look_from,
            look_at: look_at,
            vup: vup,
            defocus_angle: self.defocus_angle,
            focus_distance: self.focus_distance,
            defocus_disc_u: defocus_disc_u,
            defocus_disc_v: defocus_disc_v
        }
    }

    fn ray_clr(r: &Ray, world: &Hittable, max_bounces: i32) -> Colour {
        if max_bounces <= 0 { return colour(0, 0, 0) /*black*/ } // stop calculating rays at max depth reached
        if let Some(rec) = world.hit(r, interval(0.001, INFINITY)) {
            let scattering = rec.material.scatter(r, &rec);
            if scattering.is_some() {
                let (scatrd_ray, attenuation) = scattering.unwrap();
                return attenuation * Self::ray_clr(&scatrd_ray, world, max_bounces - 1);
            }
            return colour(0, 0, 0);
        }
    
        let uni_dir = unit_vector(r.dir());
        let a = 0.5 * (uni_dir.y() + 1.0);
    
        (1.0 - a) * colour(1, 1, 1) + a * colour(0.5, 0.7, 1.0)
    }
    
    pub fn render(&self, world: &Hittable) {
        let now = Instant::now();
        let mut imfile = File::create("image.ppm").unwrap();

        writeln!(&mut imfile, "P3");
        writeln!(&mut imfile, "{} {}", self.image_width, self.image_height);
        writeln!(&mut imfile, "255");
    
        let mut stdout = stdout();
        println!("\n\nBeginning render...");

        let write_file = Arc::new(Mutex::new(BufWriter::new(imfile)));

        for j in 0..self.image_height {
            let frac_done = ((j as f64 / self.image_height as f64) * 40.0) as usize;
            print!("\r[{char:=>width$}{dhar: >left$}] | lines left: {ll:0>3} ",
                     ll=self.image_height-j-1, char=">", 
                     width=frac_done, dhar="", left=39-frac_done
            );
            let _ = stdout.flush();

            for i in 0..self.image_width {
                let pixclr = (0..self.sample_rate).into_par_iter().map(|_s| -> Vec3 { // 100 samples per pixel
                    let mut rng = thread_rng();
                    let r = self.get_ray(i, j, &mut rng);
                    Self::ray_clr(&r, world, self.max_ray_bounces)
                }).sum::<Vec3>();

                let mut out_file = write_file.lock().unwrap();
                write_colour((1.0/self.sample_rate as f64)*pixclr, &mut out_file);
            }
        }

        write_file.lock().unwrap().flush().unwrap();

        let elapsed = now.elapsed().as_secs();
        println!("\nFinished render! Took {}min {}sec.", elapsed / 60, elapsed % 60)
    }

    fn get_ray(&self, i: i32, j: i32, rng: &mut ThreadRng) -> Ray {
        let offset = sample_square(rng);
        let pixel_sample = self.pixel00
                            + (i as f64 + offset.x())*self.delta_u
                            + (j as f64 + offset.y())*self.delta_v;
        
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disc_sample() };

        ray(ray_origin, pixel_sample - ray_origin)
    }

    fn defocus_disc_sample(&self) -> Vec3 {
        let p = randvec_in_unit_disc();
        self.center + (p.x() * self.defocus_disc_u) + (p.y() * self.defocus_disc_v)
    }
}

fn sample_square(rng: &mut ThreadRng) -> Vec3 {
    let rand1: f64 = rng.gen();
    let rand2: f64 = rng.gen();
    vec3(rand1 - 0.5, rand2 - 0.5, 0)
}
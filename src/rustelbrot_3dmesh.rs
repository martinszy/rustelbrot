//encoding=utf-8
// 3d Fractal generator
// Mandelbrot set calculated according to http://www.hiddendimension.com/FractalMath/Divergent_Fractals_Main.html
// None of the code was copied, all custom made
// Created by Faras, 2017 & 2018
// Released under GPLv3 license

extern crate palette;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate ncollide;

use Config;

use std::f64;
use std::f64::consts::E;
use std::time::Instant;
// use std::path::Path;

use std::rc::Rc;
use std::cell::RefCell;

use kiss3d::resource::{Material};
use kiss3d::builtin::NormalsMaterial;

use self::palette::{Rgb, Hsv, RgbHue,Gradient};

use self::na::{Vector3, Point3, UnitQuaternion};
use self::kiss3d::window::Window;
use self::kiss3d::light::Light;
// use kiss3d::resource::Texture;

// use ncollide::ncollide_procedural::TriMesh;
use self::ncollide::ncollide_procedural::quad_with_vertices;
// use self::ncollide::ncollide_procedural::quad;
// use ncollide::math::Point as P;

// this function tries to determine at which speed does the recursive function blow up
fn unbound_speed(x: f64,y: f64) -> (f64,f64) {
    let mut z0 = 0.0;
    let mut z1 = 0.0;
    let mut s2 = 0.0;
    let mut s3 = 0.0;
    let iterations_per_pixel = 80;
    let mut i = 0;

    'lo: loop {
        let (z2,z3) = recursive(z0,z1,x,y);
        if z2 == z0 || z2.is_nan() || z2 > 4.0 {
            break 'lo;
        }

        // let p = E**&((&z2+&z3).abs()*-1.0); //e^-ri
        let p2 = E**&((&z2).abs()*-1.0); //derivada ?
        let p3 = E**&((&z3).abs()*-1.0); //derivada ?

        z0 = z2;
        z1 = z3;
        // println!("u z2 {} z3 {} ",z2,z3);
        s2 = s2 + p2;
        s3 = s3 + p3;

        i = i+1;
        if i > iterations_per_pixel {
            break 'lo;
        }
    }

    return (s2,s3)
}

// the recursive function is the one needed for the mandelbrot set, it operates on complex numbers (actually, two tuples)
fn recursive(zr:f64,zi:f64,cr:f64,ci:f64) -> (f64,f64) {
    // formula: zn+1 = z2n + c

    // Check for numbers too large or small to handle
    // if (z0 > 0.0 && z0 < 1e32) || (z0 < 0.0 && z0 > -1e32) || (z0 == 0.0) {
    let z2r = zr * zr - zi*zi;
    let z2i = zr * zi + zr * zi;
    // }
    // else {
    //     return (z0,z1);
    // }

    // addition: (a + bi) + (c + di) = (a + c) + (b + d)i
    let z2cr = z2r+cr;
    let z2ci = z2i+ci;

    // println!("{:?}",z2cr);

    return (z2cr,z2ci)
}

fn map_range_log(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 +
        (s - from_range.0) *
        (to_range.1 - to_range.0) /
        (from_range.1 - from_range.0) *
        (
            &2.0 -
            (s - from_range.0) /
            (from_range.1 - from_range.0)
        )
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 +
        (s - from_range.0) *
        (to_range.1 - to_range.0) /
        (from_range.1 - from_range.0)
}

pub fn main(config:Config) {
    let balls = config.balls;
    let mesh = config.mesh;
   // falta cli parser https://crates.io/crates/clap
    let start = Instant::now();

    let mut window = Window::new("Rustelbrot3d");

    window.set_light(Light::StickToCamera);

    let boxi = config.boxstart;

    let precissionx:f64 = (&boxi[1]-&boxi[0])/&(config.dimentions[0]) * &config.pixelsize;
    let precissiony:f64 = (&boxi[3]-&boxi[2])/&(config.dimentions[1]) * &config.pixelsize;

    let hue_shift = 0.0;

    let gradient:Gradient<Hsv> = Gradient::new(vec![
         Hsv::new(RgbHue::from(-90.0+hue_shift), 1.0, 1.0)
        ,Hsv::new(RgbHue::from(-80.0+hue_shift), 0.4, 0.4)
        ,Hsv::new(RgbHue::from(-70.0+hue_shift), 0.5, 0.5)
        ,Hsv::new(RgbHue::from(-61.0+hue_shift), 0.6, 0.6)
        ,Hsv::new(RgbHue::from(-50.0+hue_shift), 0.7, 0.7)
        ,Hsv::new(RgbHue::from(-20.0+hue_shift), 0.8, 0.8)
        ,Hsv::new(RgbHue::from( -0.0+hue_shift), 1.0, 0.7)
        ,Hsv::new(RgbHue::from( 10.0+hue_shift), 0.5, 0.7)
        ,Hsv::new(RgbHue::from( 50.0+hue_shift), 0.2, 0.9)
        ,Hsv::new(RgbHue::from( 61.0+hue_shift), 0.1, 1.0)
        ]
    );

//    println!("hs{:?}", hue_shift);
    // //
    // println!("py{:?}", precissiony);
    // println!("px{:?}", precissionx);

    let mut x:f64 = boxi[0];

    let mut vertices = vec![];
    let p2 = Point3::new(1.0,1.0,1.0);
    let v = Vector3::new(1.0,1.0,1.0);

    while x <= boxi[1] {
        // println!("{}",x);
        let mut y:f64 = boxi[2];
        while y <= boxi[3] {

            let realx = map_range((boxi[0],boxi[1]),(0.0,1.0),x);
            let realy = map_range((boxi[2],boxi[3]),(0.0,1.0),y);

            let (s2,s3) = unbound_speed(x,y);

            let mut z1 = map_range((-1e2 as f64,-1e1 as f64),(0.1,0.2),s3/10.0);


            //
            // if z1.is_nan() {
            //     z1 = 0.01
            // }
            //
            //Limit max depth
            if z1 < 0.0 {
                z1 = map_range_log((-1e30 as f64,-1e2 as f64),(0.1,0.2),s2);
                // if z1 < 0.0 {
                //     println!("a{}",z1);
                //     z1 = -0.0;
                // }

            }
            // if z1 > 0.6 {
            //     println!("m{}",z1);
            //     z1 = 0.6
            // }

            // let p = Point3::new(realx as f32,realy as f32,(z1) as f32);

            let p = Point3::new((realx-0.5) as f32,(realy-0.5) as f32,(z1) as f32);

            // print!("{}",p);
            // let p = Point3::new(realx as f32,realy as f32,z1 as f32);
            // let pmesh = p;//Point3::new(0.0,0.0,1.0);

            let spheresize = ((1.0/config.dimentions[0]*config.pixelsize)/1.0)as f32;
            // println!("spheresize{}",spheresize);

            if balls {
                let mut a = window.add_sphere(spheresize);
                a.reorient(&p,&p2,&v);

                let hsv = gradient.get(z1 as f32);
                let rgb: Rgb = Rgb::from(hsv);
                // println!("rgb{:?}",rgb);
                a.set_color(rgb.red, rgb.green, rgb.blue);

            }

            if mesh {
                vertices.push(p);
            }

            y+=precissiony;
        }
        x+=precissionx;
    }
    //println!("vertices:{:?}",vertices );

    let mut m;

    if mesh {
        let quad = quad_with_vertices(&vertices,(config.dimentions[0]/(config.pixelsize)) as usize,(config.dimentions[1]/config.pixelsize) as usize);
        //let mut quad = quad(1.0,1.0,(config.dimentions[0]/(config.pixelsize)) as usize,(config.dimentions[1]/config.pixelsize) as usize);
        //println!("quad:{:?}",quad );
        m = window.add_trimesh(quad.clone(),Vector3::new(1.0,1.0,1.0));

        // https://github.com/sebcrozet/kiss3d/blob/master/examples/custom_material.rs
        let material   = Rc::new(RefCell::new(Box::new(NormalsMaterial::new()) as Box<Material + 'static>));

        let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

        // m.set_texture_from_file(&Path::new("/var/www/matherial/rustelbrot/generated/rustelbrot_f050.png"),&"textura");
      //  let mut current_frame = config.frames;

        //if current_frame >= config.frames {

                //let mut quad = quad.clone();
                //quad.coords = vertices.clone();
                //m = window.add_trimesh(quad,Vector3::new(1.0,1.0,1.0));
                m.set_material(material.clone());
                m.recompute_normals();
                //
                let p = Point3::new(0.0,0.0,0.0);
                let p2 = Point3::new(0.0,0.0,1.0);
                let v = Vector3::new(0.9,0.9,0.9);
                m.reorient(&p,&p2,&v);
                //m.prepend_to_local_rotation(&rot);

                // current_frame -= 1.0;
        while window.render() {
            }

        // let hue_shift = map_range((0.0,config.frames-1.0),(-180.0,180.0),current_frame) as f32;
        // let mut current_frame:f64 = config.frames - 1.0;
        // while current_frame >= 0.0 {
        //     // let frame_start = Instant::now();
        //
        //     // let surface = ImageSurface::create(Format::ARgb32, config.dimentions[0] as i32, config.dimentions[1] as i32).expect("Can't create surface");
        //     //  let cr = Context::new(&surface);
        //
        //
        //     boxi = [
        //      map_range_log((0.0,config.frames-1.0),(config.boxstart[0],config.boxend[0]),current_frame),
        //      map_range_log((0.0,config.frames-1.0),(config.boxstart[1],config.boxend[1]),current_frame),
        //      map_range_log((0.0,config.frames-1.0),(config.boxstart[2],config.boxend[2]),current_frame),
        //      map_range_log((0.0,config.frames-1.0),(config.boxstart[3],config.boxend[3]),current_frame)
        //     ];
        //
        //
        //     //
        //     // while window.render() {
                //
        //     // }
        //
        //
        // }
    //}


    let duration = start.elapsed().as_secs() as f64 + start.elapsed().subsec_nanos() as f64  * 1e-9;

    println!("Init time {} seconds until first render.",duration );


    }

}

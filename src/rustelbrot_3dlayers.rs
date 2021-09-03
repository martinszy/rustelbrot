//encoding=utf-8
// 3d Fractal generator
// Mandelbrot set calculated according to http://www.hiddendimension.com/FractalMath/Divergent_Fractals_Main.html
// None of the code was copied, all custom made
// Created by Faras, 2017 & 2018
// Released under GPLv3 license

extern crate cairo;

extern crate palette;
extern crate kiss3d;
extern crate ncollide3d;

use crate::Config;

use std::f64;
use std::usize;
use std::f64::consts::E;
use std::fs;
use std::time::Instant;
// use std::env;
// use std::rc::Rc;
// use std::path::Path;

use self::cairo::{Context, Format, ImageSurface};
use self::palette::{Rgb, Hsv, RgbHue,Gradient};

// use self::na::{Vector3, Point3, Real};
// use self::kiss3d::window::Window;
// use self::kiss3d::light::Light;

struct Layer {
    index: f64,
    cr: Context,
    surface: ImageSurface,
}


// this function tries to determine at which speed does the recursive function blow up
fn unbound_speed(x: f64,y: f64) -> usize {
    let mut z0 = 0.0;
    let mut z1 = 0.0;
    let mut s = 0.0;
    let iterations_per_pixel = 800;
    let mut i = 0;

    'lo: loop {
        let (z2,z3) = recursive(z0,z1,x,y);
        if z2 == z0 || z2.is_nan() || z2 > 4.0 {
            break 'lo;
        }
        z0 = z2;
        z1 = z3;

        let p = E**&((&z2+&z3).abs()*-1.0);
        // println!("u z2 {} z3 {} ",z2,z3);
        s = s + p;
        // s = i;

        // To get one number from a complex, do the squere root of both numbers square
        // Return the number of iterations until it bailed out if it did
        // If it didn't, draw black ...
        // Return 1-(1/(e"(abs(x))))
        //

        i = i+1;
        if i > iterations_per_pixel {
            return i
        }
    }

    return i
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
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0) * (&2.0 - (s - from_range.0) / (from_range.1 - from_range.0))
}
fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}


pub fn main(config:Config) {
    let start = Instant::now();

    let mut current_layer:f64 = 0.0;
    let mut corresponding_layer:f64;
    let mut layers = vec![];

    let boxi:[f64;4];

    let frame_start = Instant::now();

    while current_layer < config.frames {
        let is = ImageSurface::create(Format::ARgb32, config.dimentions[0] as i32, config.dimentions[1] as i32).expect("Can't create surface");

        let layer = Layer {
            index: current_layer,
            cr: Context::new(&is),
            surface: is,
        };
        layers.push(layer);
        current_layer+=1.0;
    }


    boxi = config.boxstart;

    let precissionx:f64 = (&boxi[1]-&boxi[0])/&(config.dimentions[0]) * &config.pixelsize;
    let precissiony:f64 = (&boxi[3]-&boxi[2])/&(config.dimentions[1]) * &config.pixelsize;


    //    println!("hs{:?}", hue_shift);

    // //
    // println!("py{:?}", precissiony);
    // println!("px{:?}", precissionx);

    let mut x:f64 = boxi[0];



    while x <= boxi[1] {
        // println!("{}",x);
        let mut y:f64 = boxi[2];
        while y <= boxi[3] {

            let z = unbound_speed(x,y) as f64;

            //let mut z1 = map_range((-1e2 as f64,-1e1 as f64),(0.1,0.2),z);
            let z1 = map_range_log((0.0,800.0),(0.0,1.0),z);
            //
            // //Limit max depth
            // if z1 < 0.0 {
            //     z1 = map_range_log((-1e308 as f64,-1e2 as f64),(0.0,0.2),z);
            //     if z1 < 0.0 {
            //         println!("a{}",z1);
            //         z1 = -0.0;
            //     }
            //
            // }
            //
            //

            // corresponding_layer = (z1*config.frames*(1.0/0.2)-(z1*2.5)).round();
            // let corresponding_layer = map_range_log((0.0,800.0),(0.0,config.frames-1.0),z).round();
            // corresponding_layer = (z1*(config.frames-1.0)*15.0)-4.0;
            // corresponding_layer = (z1*(config.frames-1.0));

            if z1<0.5 {
                corresponding_layer = (z1*(config.frames-1.0)*18.0)-5.0;
            }
            else {
                corresponding_layer = z1*(config.frames-1.0)*1.0;
            }


            // Sinusolidal
            // corresponding_layer = (f64::sin(z1*f64::pi())*config.frames).round();

            // Log
            // if z1<0.7 {
            //     corresponding_layer = map_range_log((0.0,0.7),(0.0,config.frames),z1);
            // }
            // else {
            //     corresponding_layer = map_range_log((0.7,1.0),(0.0,config.frames-1.0),z1);
            // }

            // if corresponding_layer >= config.frames {
            //     corresponding_layer = (config.frames-1.0).round();
            // }
            // println!("z: {} - corresponding layer: {}",z1,corresponding_layer as usize);

            // let hue_shift = 25.0*corresponding_layer as f32;
            // let light_shift = -0.03*corresponding_layer as f32;

            // let gradient:Gradient<Hsv> = Gradient::new(vec![
            //      Hsv::new(RgbHue::from(-10.0+hue_shift), 1.0, 1.0-light_shift)
            //     ,Hsv::new(RgbHue::from(-80.0+hue_shift), 0.4, 0.4-light_shift)
            //     ,Hsv::new(RgbHue::from(-7.0+hue_shift), 0.5, 0.5-light_shift)
            //     ,Hsv::new(RgbHue::from(-6.0+hue_shift), 0.6, 0.6-light_shift)
            //     ,Hsv::new(RgbHue::from(-5.0+hue_shift), 0.7, 0.7-light_shift)
            //     ,Hsv::new(RgbHue::from(-2.0+hue_shift), 0.8, 0.8-light_shift)
            //     ,Hsv::new(RgbHue::from( -0.0+hue_shift), 1.0, 0.7-light_shift)
            //     ,Hsv::new(RgbHue::from( 1.0+hue_shift), 0.5, 0.7-light_shift)
            //     ,Hsv::new(RgbHue::from( 5.0+hue_shift), 0.2, 0.8-light_shift)
            //     ,Hsv::new(RgbHue::from( 15.0+hue_shift), 0.1, 0.6-light_shift)
            //     ]
            // );

            // let hsv = gradient.get(z1 as f32);
            let rgb: Rgb = Rgb::new(40.0,40.0,40.0);

            if corresponding_layer as usize >= config.frames as usize {
                corresponding_layer = 0.0;
            }

            let fill:bool = true;
            if fill != true {
                layers[corresponding_layer as usize].cr.set_source_rgb(rgb.red as f64,rgb.green as f64,rgb.blue as f64);

                layers[corresponding_layer as usize].cr.rectangle(
                    map_range((boxi[0],boxi[1]),(0.0,config.dimentions[0]),x),
                    map_range((boxi[2],boxi[3]),(0.0,config.dimentions[1]),y),
                    config.pixelsize,
                    config.pixelsize
                );
                layers[corresponding_layer as usize].cr.fill();
            }
            else {
                let mut painted_layer = corresponding_layer;
                while painted_layer > 0.0 {
                    layers[painted_layer as usize].cr.set_source_rgb(rgb.red as f64,rgb.green as f64,rgb.blue as f64);
                    
                    layers[painted_layer as usize].cr.rectangle(
                        map_range((boxi[0],boxi[1]),(0.0,config.dimentions[0]),x),
                        map_range((boxi[2],boxi[3]),(0.0,config.dimentions[1]),y),
                        config.pixelsize,
                        config.pixelsize
                    );
                    layers[painted_layer as usize].cr.fill();

                    painted_layer = painted_layer - 1.0;
                }

            }
            y+=precissiony;
        }
        x+=precissionx;
    }



    // let gradient_height:f64 = 40.0;
    //
    // let yf:f64 = HEIGHT  - gradient_height;
    //
    // let mut xf:f64 = 0.0;
    // // println!("gradient {},{:?}",yf,gradient);
    //
    // while xf <= WIDTH {
    //     draw_gradient(&cr,&gradient,xf,yf,gradient_height);
    //     xf+=2.0;
    // }
    //
    match fs::create_dir("layers") {
        Ok(_) => println!("{} created","layers"),
        Err(_) => println!("Error create {}","layers"),
    }
    for layer in &layers {
        let filename = &format!("layers/rustelbrot_layer{:02}.png",layer.index);
        // let filename = "m.png";
        let mut file = fs::File::create(filename).expect("Couldn't create file");
        match layer.surface.write_to_png(&mut file) {
            Ok(_) => println!("{} created",filename),
            Err(_) => println!("Error create {}",filename),
        }
    }


    let duration = frame_start.elapsed().as_secs() as f64 + frame_start.elapsed().subsec_nanos() as f64  * 1e-9;
    let total_pixels = config.dimentions[0] * config.dimentions[1] / config.pixelsize;

    let pixels_per_second = total_pixels/duration;


    println!("Frame duration {} seconds. Pixels per second: {} Total pixels:{}",duration,pixels_per_second,total_pixels );
/*
    let mut window = Window::new("Rustelbrot3d");

    window.set_light(Light::StickToCamera);

    // let mut vertices = vec![];
    // let p2 = Point3::new(1.0,1.0,1.0);
    // let v = Vector3::new(1.0,1.0,1.0);

    // let mut currentvertex = 0;

    // println!("vertices:{:?}",vertices );

    // let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    // let mut c      = window.add_cube(0.10, 0.10, 0.10);

    // let texture:Rc<Texture> = Texture::new();

    //Esta es la qeu va
    // let quad = quad_with_vertices(&vertices,WIDTH as usize,HEIGHT as usize);
    // let mut m      = window.add_trimesh(quad,Vector3::new(1.0,1.0,1.0));
    // m.set_texture_from_file(&Path::new("/var/www/matherial/rustelbrot/rustelbrot_f050.png"),&"textura");


    //
    // while window.render() {
    //     m.prepend_to_local_rotation(&rot);
    // }
    let p2 = Point3::new(1.0,1.0,1.0);
    let v = Vector3::new(1.0,1.0,1.0);

    for layer in &layers {
        let mut m = window.add_quad(1.0, 1.0, 1, 1);

        let p = Point3::new(0.0,0.0,(layer.index * 0.1) as f32);

        m.reorient(&p,&p2,&v);
        // m.enable_backface_culling(false);

        let filename = &format!("/var/www/matherial/rustelbrot/rustelbrot_layer{:02}.png",layer.index);
        let texturename = &format!("textura{:02}.png",layer.index);

        // println!("{:?}", filename);

        m.set_texture_from_file(&Path::new(filename),&texturename);

    }
*/

    let duration = start.elapsed().as_secs() as f64 + start.elapsed().subsec_nanos() as f64  * 1e-9;

    println!("Total duration {} seconds.",duration );
/*
    while window.render() {

    }
*/
}

//encoding=utf-8
// 2D Fractal generator
// Mandelbrot set calculated according to http://www.hiddendimension.com/FractalMath/Divergent_Fractals_Main.html
// None of the code was copied, all custom made
// Created by Faras, 2017 & 2018
// Released under GPLv3 license

extern crate cairo;
extern crate palette;

use crate::Config;

use std::f64;
use std::f64::consts::E;
use std::fs;
use std::time::Instant;
// use std::env;

use self::cairo::{Context, Format, ImageSurface};
use self::palette::{Rgb, Hsv, RgbHue,Gradient};

// this function tries to determine at which speed does the recursive function blow up
fn unbound_speed(x: f64,y: f64) -> f64 {
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
            return i as f64
        }
    }

    return i as f64
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

// this funtion maps each complex value to an x,y point and performs color operations on the z value that comes from the unbound_speed function
fn draw(cr:&Context,boxi:&[f64],config:&Config,gradient:&Gradient<Hsv>, x: f64,y: f64,z: f64) {
    // println!("draw x{} y{} z{}",x,y,z);
    //
    // let mut z1 = map_range((-1e3 as f64,1e3 as f64),(0.0,2.0),z);
    let z1 = map_range_log((0.0,800.0),(1.0,-1.0),z);

    // if z1.is_nan() {
    //     z1 = 0.01
    //
    // }


    let hsv = gradient.get(z1 as f32);
    let rgb: Rgb = Rgb::from(hsv);

    cr.set_source_rgb(rgb.red as f64,rgb.green as f64,rgb.blue as f64);

    cr.rectangle(
        map_range((boxi[0],boxi[1]),(0.0,config.dimentions[0]),x),
        map_range((boxi[2],boxi[3]),(0.0,config.dimentions[1]),y),
        config.pixelsize,
        config.pixelsize
    );
    cr.fill();

}


fn draw_gradient(cr:&Context,width:f64,gradient:&Gradient<Hsv>, x: f64,y: f64,gradient_height:f64) {
    // println!("draw_gradient x{} y{} gradient_height{}",x,y,gradient_height);

    let z1 = map_range((0.0,width),(0.0,1.0),x);
    let hsv = gradient.get(z1 as f32);
    let rgb: Rgb = Rgb::from(hsv);

    cr.set_source_rgb(rgb.red as f64,rgb.green as f64,rgb.blue as f64);

    cr.rectangle(
        x,
        y,
        2.0,
        gradient_height
    );
    cr.fill();
}


fn map_range_log(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0) * (&2.0 - (s - from_range.0) / (from_range.1 - from_range.0))
}
fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub fn main(config:Config) {
   // falta cli parser https://crates.io/crates/clap
    let start = Instant::now();

    let mut current_frame:f64 = config.frames - 1.0;

    let mut boxi:[f64;4];

    while current_frame >= 0.0 {
        // let frame_start = Instant::now();

        let surface = ImageSurface::create(Format::ARgb32, config.dimentions[0] as i32, config.dimentions[1] as i32).expect("Can't create surface");
         let cr = Context::new(&surface);


        boxi = [
         map_range_log((0.0,config.frames-1.0),(config.boxstart[0],config.boxend[0]),current_frame),
         map_range_log((0.0,config.frames-1.0),(config.boxstart[1],config.boxend[1]),current_frame),
         map_range_log((0.0,config.frames-1.0),(config.boxstart[2],config.boxend[2]),current_frame),
         map_range_log((0.0,config.frames-1.0),(config.boxstart[3],config.boxend[3]),current_frame)
        ];

        //Hardcodeo que no haya animacion
        // boxi = config.boxstart;


        let precissionx:f64 = (&boxi[1]-&boxi[0])/&(config.dimentions[0]) * &config.pixelsize;
        let precissiony:f64 = (&boxi[3]-&boxi[2])/&(config.dimentions[1]) * &config.pixelsize;


        let hue_shift = 0.0; // map_range((0.0,config.frames-1.0),(-180.0,180.0),current_frame) as f32;


        let gradient:Gradient<Hsv> = Gradient::new(vec![
             Hsv::new(RgbHue::from(-90.0+hue_shift), 0.1, 0.1)
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

        //
        // println!("py{:?}", precissiony);
        // println!("px{:?}", precissionx);

        let mut x:f64 = boxi[0];

        while x <= boxi[1] {
            // println!("{}",x);
            let mut y:f64 = boxi[2];
            while y <= boxi[3] {
                // println!("{}",y);
                let z = unbound_speed(x,y);

                draw(&cr,&boxi,&config,&gradient,x,y,z);
                y+=precissiony;
            }
            x+=precissionx;
        }


        let gradient_height:f64 = 40.0;

        let yf:f64 = config.dimentions[1]  - gradient_height;

        let mut xf:f64 = 0.0;
        // println!("gradient {},{:?}",yf,gradient);

        while xf <= config.dimentions[0] {
            draw_gradient(&cr,config.dimentions[0],&gradient,xf,yf,gradient_height);
            xf+=2.0;
        }

        let filename = &format!("generated/rustelbrot_f{:03}_box{}x{}x{}x{}.png",current_frame,boxi[0],boxi[1],boxi[2],boxi[3]);
        // let filename = "m.png";
        match fs::create_dir("generated") {
            Ok(_) => println!("{} created",filename),
            Err(_) => println!("Error create {}",filename),
        }
        let mut file = fs::File::create(filename).expect("Couldn't create file");
        match surface.write_to_png(&mut file) {
            Ok(_) => println!("{} created",filename),
            Err(_) => println!("Error create {}",filename),
        }


        // let duration = frame_start.elapsed().as_secs() as f64 + frame_start.elapsed().subsec_nanos() as f64  * 1e-9;
        // let total_pixels = config.dimentions[0] * config.dimentions[1] / config.pixelsize;

        // let pixels_per_second = total_pixels/duration;


        // println!("Frame {} duration {} seconds. Pixels per second: {} Total pixels:{}",current_frame,duration,pixels_per_second,total_pixels );

        current_frame -= 1.0;
    }

    let duration = start.elapsed().as_secs() as f64 + start.elapsed().subsec_nanos() as f64  * 1e-9;

    let frames_per_second = config.frames/duration;

    println!("Total duration {} seconds. Frames per second: {} Total frames:{}",duration,frames_per_second,config.frames );


}

// 2D Fractal generator
// Mandelbrot set calculated according to http://www.hiddendimension.com/FractalMath/Divergent_Fractals_Main.html
// None of the code was copied, all custom made
// Created by Faras, 2017 & 2018
// Released under GPLv3 license
// TODO:
// - CLI version should recieve width,height,boxi and pixel size as command line parameters
// - GUI should allow to change the paramters and redraw

mod mandelbrot;
// mod mandelbrot3d;
mod mandelbrot3dmesh;
mod mandelbrotvid;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {

    let matches = App::new("Mandelbrot")
                          .version("1.0")
                          .author("Faras. <faras@faras.me>")
                          .about("Generador de fractales mandelbrot")
                        //   .args_from_usage(
                        //       "-c, --config=[FILE] 'Sets a custom config file'
                        //       <INPUT>              'Sets the input file to use'
                        //       -v...                'Sets the level of verbosity'")
                            .subcommand(SubCommand::with_name("3dmesh")
                                        .about("genera en 3d mesh"))
                            .subcommand(SubCommand::with_name("3d")
                                        .about("genera en 3d"))
                            .subcommand(SubCommand::with_name("2d")
                                        .about("genera en 2d"))
                                    //   .arg_from_usage("-d, --debug 'Print debug information'"))
                          .get_matches();

    if let Some(_matches) = matches.subcommand_matches("3d") {
        println!("3d");
        // mandelbrot3d::main();
    }
    else if let Some(_matches) = matches.subcommand_matches("3dmesh") {
        println!("3dmesh");
        mandelbrot3dmesh::main();
    }
    else if let Some(_matches) = matches.subcommand_matches("vid") {
        println!("video");
        mandelbrotvid::main();
    }
    else {
        println!("2d");
        mandelbrot::main();
    }
}

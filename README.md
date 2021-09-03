Website: http://faras.me/fractal

# Demos
* Video: http://faras.me/fractal/felicidad%20infinita.mp4
* Layers: http://faras.me/fractal/capas/
* Object with 3d Layers: http://faras.me/fractal/objeto.mp4


# Install
<<<<<<< HEAD
* Install Rust [https://www.rust-lang.org/en-US/install.html] (tested in version 1.33.0)
* Checkout this repo

# Run
'cargo run [COMMAND] [FLAGS] [OPTIONS]'

COMMAND can be one of g3d (generates 3d meshes or 2d layers), vid (generates a sequence of 2d frames) or g2d (generates a sequence of 2d frames, default)

```
USAGE:
    rustelbrot [COMMAND] [FLAGS] [OPTIONS] 

COMMANDS:
    g2d     genera en 2d frames para video
    g3d     genera en 3d (actualmente no anda bien)
    help    Prints this message or the help of the given subcommand(s)
    vid     genera en 2d
=======
* Install Rust [https://www.rust-lang.org/en-US/install.html]
* Checkout this repo

# Run
'cargo run [FLAGS] [OPTIONS] [SUBCOMMAND]'

FLAGS can be one of g3d (generates 3d meshes or 2d layers), vid (generates a sequence of 2d frames) or g2d (generates a sequence of 2d frames, default)

```
USAGE:
    rustelbrot [FLAGS] [OPTIONS] [SUBCOMMAND]
>>>>>>> pr1

FLAGS:
    -h, --help               Prints help information
    -o, --output-template    Formato del nombre de salida, incluyendo carpeta y número de cuadro
<<<<<<< HEAD
    -V, --version            Prints version information

OPTIONS:
    -e, --boxend <boxend>            Define las 4 coordenadas de la caja del cuadro final separadas por x [x1,x2,y1,y2]
    -s, --boxstart <boxstart>        Define las 4 coordenadas de la caja del cuadro inicial separadas por x [x1,x2,y1,y2]
    -d, --dimentions <dimentions>    Tamaño de las imágenes a generar WIDTHxHEIGHT en pixeles
    -f, --frames <frames>            Define la cantidad de cuadros a generar
    -p, --pixelsize <pixelsize>      Tamaño del pixel, aumentar este valor disminuye el detalle y aumenta la performance

```

# Notes
* Default 2d mode and vid will generate 100 PNG images in current directory, parameters can be adjusted in the code.

=======
    			     Output name format, including folder and frame number.
    -V, --version            Prints version information

OPTIONS:
    -e, --boxend <boxend>            Define las coordenadas de la caja del cuadro final [x1,y1,x2,y2]
    		 		     The coordinates of the final frame box [x1,y1,x2,y2]
    -s, --boxstart <boxstart>        Define las coordenadas de la caja del cuadro inicial [x1,y1,x2,y2]
    		   		     The coordinates of the starting square box [x1,y1,x2,y2]
    -d, --dimentions <dimentions>    Tamaño de las imágenes a generar WIDTHxHEIGHT en pixeles
    		     		     The size of the images to be generated: WIDTH x HEIGHT in pixels
    -f, --frames <frames>            Define la cantidad de cuadros a generar
    		 		     The number of frames to generate.
    -p, --pixelsize <pixelsize>      Tamaño del pixel, aumentar este valor disminuye el detalle y aumenta la performance
    		    		     Pixel size: Increasing this value decreases detail and increases performance.

SUBCOMMANDS:
    g2d     genera en 2d frames para video
    	    Generates 2d frames for video
    g3d     genera en 3d (actualmente no anda bien)
    	    Generates 3d frames (currently not working well)
    help    Prints this message or the help of the given subcommand(s)
    vid     genera en 2d
    	    Generates 2d
```

>>>>>>> pr1

# Example commands
* 3d layers, nice spot: 'cargo run g3d -l -s 0.33x0.40x0.05x0.15'
* Same, but 30 layers and 25cm 720dpi: 'cargo run g3d -l -d 7087x7087 -f 30  -s 0.33x0.40x0.05x0.15'

# Prototype 2
* 25 layers in 20x20cm 720dpi 'cargo run g3d -l -d 5670x5670 -f 25'
* Printed on acrylic crystal 200x200x2mm
* Backlight with LED strip

<<<<<<< HEAD
# 3d printed layers prototype
* cargo run g3d -l --boxstart 0.367330x0.367340x0.141790x0.141800 -d 1000x1000 -f 26
* this creates only 10 good layers, from 15 to 24, move those to a folder, access the folder and run
* mogrify -format ppm *.png
* potrace rustelbrot_layer*.ppm -s -t 15 -n -i

This creates 15 svg files, that you can then extrude and turn into gcode.
=======
# Notes
* Default 2d mode and vid will generate 100 PNG images in current directory, parameters can be adjusted in the code.
>>>>>>> pr1

# TODO:
- GUI
- STL export for 3d mode

# License
GPLv3

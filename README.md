# Install
* Install Rust [https://www.rust-lang.org/en-US/install.html]
* Checkout this repo

# Run
'cargo run [FLAGS] [OPTIONS] [SUBCOMMAND'

FLAGS can be one of g3d (generates 3d meshes or 2d layers), vid (generates a sequence of 2d frames) or g2d (generates a sequence of 2d frames, default)

# Example commands
* 3d layers, nice spot: 'cargo run g3d -l -s 0.33x0.40x0.05x0.15'
* Same, but 30 layers and 25cm 720dpi: 'cargo run g3d -l -d 7087x7087 -f 30  -s 0.33x0.40x0.05x0.15'


# Notes
* Default 2d mode and vid will generate 100 PNG images in current directory, parameters can be adjusted in the code.
* 3d and 3dmesh mode don't work properly yet

# TODO:
- GUI
- Improve 3d
- Document options

#License
GPLv3

extern crate clap;
extern crate lodepng;
extern crate log4rs;
#[macro_use]
extern crate log;

use clap::{Arg, App};

/// Generate an image file to represent the world.
/// This image file will always be in PNG format, and will represent one cell in the world
/// as one pixel
fn generate_world_image(filename: &str, width: usize, height: usize) {
    let mut image: Vec<lodepng::RGB<u8>> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let r: u8 = (255f32 * ((x as f32) / (width as f32))) as u8;
            let g: u8 = (255f32 * ((y as f32) / (height as f32))) as u8;
            let b: u8 = 0;

            image.push(lodepng::RGB::<u8> {r: r, g: g, b: b});
        }
    }

    lodepng::encode_file(filename,
        &image[..],
        width,
        height,
        lodepng::ColorType::LCT_RGB,
        8)
        .ok()
        .expect("Failed to write file");
}
        
#[allow(dead_code)]
fn main() {
    log4rs::init_file("log4rs.toml", log4rs::toml::Creator::default()).unwrap();

    let matches = App::new("genworld")
        .version("0.1.0")
        .author("Graham Cox <graham@grahamcox.co.uk>")
        .about("Generate a new world as an overview map image")
        .arg(Arg::with_name("FILE")
            .short("f")
            .long("file")
            .help("Filename to generate image into")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("WIDTH")
            .short("x")
            .long("width")
            .requires("HEIGHT")
            .help("Width of the world to generate. Defaults to 10,000")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("HEIGHT")
            .short("y")
            .long("height")
            .requires("WIDTH")
            .help("Height of the world to generate. Defaults to 10,000")
            .required(false)
            .takes_value(true))
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    let width = matches.value_of("WIDTH")
        .unwrap_or("10000")
        .parse::<usize>()
        .ok()
        .expect("Width was not a valid number");
    let height = matches.value_of("HEIGHT")
        .unwrap_or("10000")
        .parse::<usize>()
        .ok()
        .expect("Height was not a valid number");

    info!("Generating world of size {}x{} into {}", width, height, filename);
    generate_world_image(filename, width, height);
}

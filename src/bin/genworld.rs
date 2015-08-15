extern crate clap;
use clap::{Arg, App};

fn main() {
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
        .parse::<u32>()
        .ok()
        .expect("Width was not a valid number");
    let height = matches.value_of("HEIGHT")
        .unwrap_or("10000")
        .parse::<u32>()
        .ok()
        .expect("Height was not a valid number");

    println!("Generating world of size {}x{} into {}", width, height, filename);
}

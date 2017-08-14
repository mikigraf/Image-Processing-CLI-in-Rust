extern crate clap;
extern crate image;
use clap::{Arg,App,SubCommand};
use std::fs::File;
use image::{FilterType, GenericImage, Pixel};
use image::DynamicImage;

fn main() {
     let matches = App::new("IPCLI")
                          .version("0.1")
                          .author("Mikolaj Wawrzyniak <mikolaj.wawrzyniak at fh-dortmund.de>")
                          .about("Basic CLI for image processing")
                          .arg(Arg::with_name("image")
                               .short("i")
                               .long("image")
                               .value_name("FILE")
                               .help("Opens specified image file and uses it for transformations.")
                               .takes_value(true))
                          .arg(Arg::with_name("save")
                               .short("s")
                               .long("save")
                               .value_name("FILE")
                               .help("Copies provided image to specified file.")
                               .takes_value(true))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let imagePath = matches.value_of("image").unwrap_or("empty");
    println!("Value for image path: {}", imagePath);

    let savePath = matches.value_of("save").unwrap_or("empty");
    println!("Value for save path: {}", savePath);

    openAndSave(imagePath, savePath);
    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
}

fn openAndSave(i: &str, s: &str){
    let img = image::open(i).expect("Opening image failed");
    let mut out = File::create(s).unwrap();
    img.save(&mut out, image::PNG).expect("Saving image failed");
}

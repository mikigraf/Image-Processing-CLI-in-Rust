extern crate clap;
extern crate image;

use clap::{Arg,App,SubCommand};
use std::fs::File;
use image::{FilterType, GenericImage, Pixel};
use image::DynamicImage;
use std::path::Path;

fn main() {
     let matches = App::new("IPCLI")
                          .version("0.1")
                          .author("Mikolaj Wawrzyniak <mikolaj.wawrzyniak at fh-dortmund.de>")
                          .about("Basic CLI for image processing")
                          .arg(Arg::with_name("operation")
                               .short("o")
                               .long("operation")
                               .help("Specifies operation to be done on the image")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("image")
                               .short("i")
                               .long("image")
                               .value_name("FILE")
                               .help("Opens specified image file and uses it for transformations.")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("save")
                               .short("s")
                               .long("save")
                               .value_name("FILE")
                               .help("Outputs transformed image into specified file OR uses the second file to calculate the perceptual hash values and outputs their difference")
                               .takes_value(true)
                               .required(true))
                          .get_matches();

    let imagePath = matches.value_of("image").unwrap_or("empty");
    println!("Transforming the image: {}", imagePath);

    let savePath = matches.value_of("save").unwrap_or("empty");
    println!("Into: {}", savePath);

    let operation = matches.value_of("operation").unwrap_or("empty");
    println!("Operation: {}", operation);

    match operation.as_ref(){
        "copy" => {openAndSave(imagePath, savePath)}
        "thumbnail" => {createThumnbail(imagePath, savePath)}
        _ => {println!("Not implemented yet!")}
    }

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
}


fn createThumnbail(i: &str, s: &str){
    let img = image::open(i).expect("Opening image failed");
    let thumbnail = img.resize(120,120, FilterType::Lanczos3);
    saveFile(&thumbnail, &i, &s);

}

fn openAndSave(i: &str, s: &str){
    let img = image::open(i).expect("Opening image failed");
    saveFile(&img, &i, &s);
}

fn saveFile(img: &DynamicImage, i: &str, s: &str){
        let mut out = File::create(s).unwrap();
        let ext: String = i.chars().skip(i.len()-3).take(3).collect();
        match ext.as_ref() {
            "jpg" => {img.save(&mut out, image::JPEG).expect("Saving image failed");}
            "png" => {img.save(&mut out, image::PNG).expect("Saving image failed");}
            _ => {println!("something else")}
        }
}


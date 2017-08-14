extern crate clap;
extern crate image;

use clap::{Arg,App,SubCommand};
use std::fs::File;
use image::{FilterType, GenericImage, Pixel};
use image::DynamicImage;
use std::path::Path;

fn main() {
     let matches = App::new("IPCLI ")
                          .version("0.1")
                          .author("Mikolaj Wawrzyniak <mikolaj.wawrzyniak at fh-dortmund.de>")
                          .about("Basic CLI for image processing")
                          .arg(Arg::with_name("operation")
                               .short("o")
                               .long("operation")
                               .help("Specifies operation to be done on the image")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("value")
                               .short("v")
                               .long("value")
                               .help("Value for the transformation. To see what values are needed, check the documentation.")
                               .takes_value(true)
                               .required(false))
                          .arg(Arg::with_name("image")
                               .short("i")
                               .long("image")
                               .value_name("FILE")
                               .help("Opens specified image file and uses it for transformations.")
                               .takes_value(true)
                               .required(true))
                          .get_matches();

    let imagePath = matches.value_of("image").unwrap_or("empty");
    println!("Transforming the image: {}", imagePath);

    let operation = matches.value_of("operation").unwrap_or("empty");
    println!("Using operation: {}", operation);

    let value = matches.value_of("value").unwrap_or("empty");
    println!("Value: {}", value);

    match operation.as_ref(){
        "copy" => {copy(imagePath)}
        "thumbnail" => {
            let size: u32 = value.parse().unwrap();
            createThumnbail(imagePath, size)}
        "blur" => {
            let v : f32 = value.parse().unwrap();
            gaussianBlur(imagePath,v)}
        "brighten" => {
            let v: i32 = value.parse().unwrap();
            brighten(imagePath,v)}
        "huerotate" => {
            let v: i32 = value.parse().unwrap();
            huerotate(imagePath,v)}
        "contrast" => {
            let v: f32 = value.parse().unwrap();
            contrast(imagePath,v);}
        "grayscale" => {
            grayscale(imagePath);
        }
        "invert" => {
            invert(imagePath);
        }
        _ => {println!("Not implemented yet!")}
    }
}


fn createThumnbail(i: &str, size: u32){
    let operation = "Thumbnail";
    let img = image::open(i).expect("Opening image failed");
    let thumbnail = img.resize(size,size, FilterType::Lanczos3);
    saveFile(&thumbnail, &i, &operation);

}

fn copy(i: &str){
    let operation = "Copy";
    let img = image::open(i).expect("Opening image failed");
    saveFile(&img, &i, &operation);
}

fn gaussianBlur(i: &str, v: f32){
    let operation = "GuassianBlur";
    let img = image::open(i).expect("Opening image failed");
    let blurred = img.blur(v);
    saveFile(&blurred, &i, &operation);
}

fn brighten(i: &str,v: i32){
    let operation = "Brighten";
    let img = image::open(i).expect("Opening image failed");
    let brightened = img.brighten(v);
    saveFile(&brightened, &i, &operation);
}

fn huerotate(i: &str, v: i32){
    let operation = "Huerotate";
    let img = image::open(i).expect("Opening image failed");
    let huerotated = img.huerotate(v);
    saveFile(&huerotated, &i, &operation);
}

fn contrast(i: &str, v: f32){
    let operation = "AdjustContrast";
    let img = image::open(i).expect("Opening image failed");
    let contrast = img.adjust_contrast(v);
    saveFile(&contrast, &i, &operation);
}

fn grayscale(i: &str){
    let operation = "Grayscale";
    let img = image::open(i).expect("Opening image failed");
    let grayscale = img.grayscale();
    saveFile(&grayscale, &i, &operation);
}

fn invert(i: &str){
    let operation = "Invert";
    let mut img = image::open(i).expect("Opening image failed");
    img.invert();
    saveFile(&img, &i, &operation);
}

fn saveFile(img: &DynamicImage, i: &str, operation: &str){
    let mut outputPath: String = i.chars().take(i.len()-4).collect();
    let ext: String = i.chars().skip(i.len()-3).take(3).collect();
    outputPath.push_str(operation);
    outputPath.push_str(".");
    outputPath.push_str(&ext);
    println!("Output path: {}", outputPath);
    let mut out = File::create(outputPath).unwrap();
    match ext.as_ref() {
        "jpg" | "JPG" => {img.save(&mut out, image::JPEG).expect("Saving image failed");}
        "png" | "PNG" => {img.save(&mut out, image::PNG).expect("Saving image failed");}
        "gif" | "GIF" => {img.save(&mut out, image::GIF).expect("Saving image failed");}
        "bmp" | "BMP" => {img.save(&mut out, image::BMP).expect("Saving image failed");}
        "ico" | "ICO" => {img.save(&mut out, image::ICO).expect("Saving image failed");}
        _ => {println!("Unsupported file format")}
    }
}


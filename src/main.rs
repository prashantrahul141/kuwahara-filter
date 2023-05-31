use clap::Parser;
use image::{DynamicImage, GenericImageView};
use std::process::exit;

// Simple program.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// input filename.
    #[arg(short = 'f', long,  value_name = "FILENAME", value_hint = clap::ValueHint::FilePath)]
    filename: String,

    /// kernel size for sampling.
    #[arg(short = 'k', long, value_name = "KERNEL")]
    kernal: u32,
}

fn main() {
    // reading command line arguments
    let _args = Args::parse();

    // check kernal value.
    if _args.kernal < 3 || _args.kernal % 2 == 0 {
        println!("[Error] Kernal cannot be smaller than 3, and cannot be divisble by 2.");
        exit(1)
    }

    // opening image
    println!("Reading Image : {}", _args.filename);
    let image: DynamicImage = match image::open(Args::parse().filename) {
        Ok(result) => result,
        Err(err) => {
            println!("[ERROR] {}", err);
            exit(1);
        }
    };

    // looping through pixels.
    for i in image.pixels() {
        print!("{:?}", i);
    }
}

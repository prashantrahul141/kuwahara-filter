use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage, Rgba};
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
    kernal: i32,
}

fn main() {
    // reading command line arguments
    let args = Args::parse();

    // check kernal value.
    if args.kernal < 3 || args.kernal % 2 == 0 {
        println!("[Error] Kernal cannot be smaller than 3, and cannot be divisble by 2.");
        exit(1)
    }
    let qdrnt_size = (args.kernal - 1) / 2;
    // opening image
    println!("Reading Image : {}", args.filename);
    let original_image: DynamicImage = match image::open(Args::parse().filename) {
        Ok(result) => result,
        Err(err) => {
            println!("[ERROR] {}", err);
            exit(1);
        }
    };

    let new_image: RgbImage = ImageBuffer::new(original_image.width(), original_image.height());

    // looping through pixels.
    for y in 0i32..original_image.height() as i32 {
        for x in 0i32..original_image.width() as i32 {
            // current pixel on the original image.
            // let current_pixel = original_image.get_pixel(x as u32, y as u32);

            // quadrants around pixels.
            let mut quadrants: Vec<Vec<Rgba<u8>>> = vec![vec![], vec![], vec![], vec![]];

            // looping through sectors for each pixel.
            for operation_y in (y - qdrnt_size)..(y + qdrnt_size) {
                if operation_y < 0
                    || operation_y >= original_image.height() as i32
                    || x == operation_y
                {
                    continue;
                }

                for operation_x in (x - qdrnt_size)..(x + qdrnt_size) {
                    if operation_x < 0
                        || operation_x >= original_image.width() as i32
                        || x == operation_x
                    {
                        continue;
                    }

                    // checking which quadrant pixel belongs to adding its color to it.
                    let qudrant_index = match (operation_x > x, operation_y > y) {
                        (true, true) => 3,
                        (false, true) => 2,
                        (true, false) => 1,
                        (false, false) => 0,
                    };

                    quadrants[qudrant_index]
                        .push(original_image.get_pixel(operation_x as u32, operation_y as u32));
                }
            }

            // calculate average of all quadrants.
        }
    }
}

use clap::Parser;
use image::{save_buffer, DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba};
use std::{process::exit, time::Instant};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// input filename.
    #[arg(short = 'f', long,  value_name = "FILENAME", value_hint = clap::ValueHint::FilePath)]
    filename: String,

    /// kernel size for sampling.
    #[arg(short = 'k', long, value_name = "KERN:EL")]
    kernel: i32,
}

// calculates and returns average color of a quadrant in Rgba() format.
fn average_quadrant_color(colors: &Vec<Rgba<u8>>) -> Rgb<u8> {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;

    let number: u32 = if colors.len() == 0 { 1 } else { colors.len() } as u32;

    for color in colors {
        r += color.0[0] as u32;
        g += color.0[1] as u32;
        b += color.0[2] as u32;
    }

    return Rgb([(r / number) as u8, (g / number) as u8, (b / number) as u8]);
}

// calculates standard deviation.
fn std_deviation(lumens: Vec<f64>) -> f64 {
    let len = lumens.len() as f64;
    let lumens_sum: f64 = lumens.iter().sum::<f64>() / len;

    let mut dist_sum: f64 = 0.0_f64;
    for each_lumen in lumens {
        dist_sum += ((each_lumen - lumens_sum).abs()).powf(2.0_f64);
    }

    let result = (dist_sum / len).sqrt();
    if result > 0.000_f64 {
        return result;
    } else {
        return 10.0_f64;
    }
}

fn main() {
    // reading command line arguments
    let args = Args::parse();

    // check kernel value.
    if args.kernel < 3 || args.kernel % 2 == 0 {
        println!("[Error] kernel cannot be smaller than 3, and cannot be divisble by 2.");
        exit(1)
    }
    let qdrnt_size = (args.kernel - 1) / 2;
    // opening image
    println!("Reading Image : {}", args.filename);
    let original_image: DynamicImage = match image::open(Args::parse().filename) {
        Ok(result) => result,
        Err(err) => {
            println!("[ERROR] {}", err);
            exit(1);
        }
    };

    // new image buffer.
    let mut new_image: RgbImage = ImageBuffer::new(original_image.width(), original_image.height());

    println!("Looping through each pixel in image.");
    let start_time = Instant::now();
    // looping through pixels.
    for y in 0i32..original_image.height() as i32 {
        for x in 0i32..original_image.width() as i32 {
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
                    let current_pixel: Rgba<u8> =
                        original_image.get_pixel(operation_x as u32, operation_y as u32);

                    quadrants[qudrant_index].push(current_pixel);
                }
            }

            // calculate average of all quadrants.
            let mut quadrant_averages: Vec<Rgb<u8>> = vec![];

            // calculate luminance.
            let mut quadrant_lumen_std_deviations: Vec<f64> = vec![];

            // looping through each quadrant.
            for quadrant in quadrants {
                quadrant_averages.push(average_quadrant_color(&quadrant));
                let mut quadrant_pixel_lumens: Vec<f64> = vec![];

                // looping through each pixel inside a quadrant.
                for each_pixel in quadrant {
                    let lumen: f64 = 0.2126 * each_pixel.0[0] as f64
                        + 0.7152 * each_pixel.0[1] as f64
                        + 0.0722 * each_pixel.0[2] as f64;

                    quadrant_pixel_lumens.push(lumen);
                }

                // calculating and saving standard deviation.
                let std_d = std_deviation(quadrant_pixel_lumens);
                quadrant_lumen_std_deviations.push(std_d);
            }

            let mut temp_qlsd = quadrant_lumen_std_deviations.clone();

            // sorting to get the minimum standard deviation.
            temp_qlsd.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let min_std_deviation = temp_qlsd[0];

            // get index of minimum deviation from original vector.
            let mut index_of_min_deviation: usize = 0;
            for qlsd in quadrant_lumen_std_deviations {
                if qlsd == min_std_deviation {
                    break;
                }
                index_of_min_deviation += 1;
            }

            // use above index to choose quadrant.
            let final_color = quadrant_averages[index_of_min_deviation];

            // finnaly putting pixel RGB values.
            new_image.put_pixel(x as u32, y as u32, final_color);
        }
    }

    println!("Done looping, took {:?}ms", start_time.elapsed());
    println!("Writing image [result.jpg].");
    // saving buffer to disk.
    match save_buffer(
        "result.jpg",
        &new_image,
        original_image.width(),
        original_image.height(),
        image::ColorType::Rgb8,
    ) {
        Err(err) => println!("[ERROR] - while saving - {}", err),
        Ok(result) => result,
    };
    println!("Done writing.");
}

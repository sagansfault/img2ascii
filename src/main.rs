use std::error::Error;

use image::GenericImageView;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let first = args.iter().skip(1).next().expect("Provide an image URL");

    let img_bytes = reqwest::get(first).await?.bytes().await?;
    let image = image::load_from_memory(&img_bytes)?;

    let image = image.resize_exact(50, 50, image::imageops::FilterType::Triangle);
    let image = image.adjust_contrast(50.0);

    let mut string: String = String::new();

    let mut current_y = 0;
    for (x, y, val) in image.grayscale().pixels() {
        if current_y != y {
            string.push('\n');
        } else {
            string.push_str(format!("{} ", to_char(val.0[0])).as_str());
        }
        current_y = y;
    }

    println!("{}", string);

    Ok(())
}

const MAX: u8 = (0.9 * 255.0) as u8;
const VERY_BRIGHT: u8 = (0.8 * 255.0) as u8;
const BRIGHT: u8 = (0.6 * 255.0) as u8;
const NORMAL: u8 = (0.4 * 255.0) as u8;
const DIM: u8 = (0.3 * 255.0) as u8;
const VERY_DIM: u8 = (0.2 * 255.0) as u8;
const MIN: u8 = (0.1 * 255.0) as u8;
fn to_char(val: u8) -> char {
    if val > MAX {
        '⠿'
    } else if val > VERY_BRIGHT {
        '⠾'
    } else if val > BRIGHT {
        '⠺'
    } else if val > NORMAL {
        '⠼'
    } else if val > DIM {
        '⠧'
    } else if val > VERY_DIM {
        '⠣'
    } else if val > MIN {
        '⠡'
    } else {
        '⠠'
    }
}

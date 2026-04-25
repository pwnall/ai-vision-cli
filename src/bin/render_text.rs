use ab_glyph::{FontRef, PxScale};
use clap::Parser;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    text: String,
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let font_bytes = include_bytes!("../../tests/testdata/font.ttf");
    let font = FontRef::try_from_slice(font_bytes).unwrap();

    let height = 200;
    let width = 800;
    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }

    let scale = PxScale::from(48.0);

    draw_text_mut(&mut image, Rgb([0, 0, 0]), 10, 50, scale, &font, &cli.text);

    image.save(cli.output).unwrap();
}

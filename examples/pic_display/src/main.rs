use ::pic_display::pic_process::{gen_map, nearest};
use image::{imageops::FilterType, ImageReader};
use std::io::Cursor;

// use base64::{engine::general_purpose::STANDARD, Engine as _};
fn main() {
    let img = ImageReader::new(Cursor::new(include_bytes!("../pics/13.jpg")))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    {
        let img = img.resize(120, 120, FilterType::Lanczos3);
        img.save("t.png").unwrap();
    }
    let img = img.resize(120, 80, FilterType::Lanczos3);

    let last_map = gen_map(&img.to_rgb8());
    let mut img = img.to_rgb8();
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pix = img.get_pixel(x, y);
            img.put_pixel(x, y, last_map[nearest(last_map, pix)]);
        }
    }
    img.save("out.png").unwrap();
}

// #[test]
// fn a() {
//     let rgb: Rgb = Rgb::from([0., 0., 0.5]);
//     let hsv: Hsv = Hsv::from_color(rgb);
//     dbg!(hsv.get_hue().into_inner());
// }

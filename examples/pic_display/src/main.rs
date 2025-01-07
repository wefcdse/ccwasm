use ::pic_display::pic_process::{gen_map, nearest};
use image::{imageops::FilterType, ImageReader};
use std::{fs, io::Cursor, path::Path};

// use base64::{engine::general_purpose::STANDARD, Engine as _};
fn main() {
    let pics = fs::read_dir("./pics").unwrap();
    let out: &Path = "./out/".as_ref();
    fs::DirBuilder::new().recursive(true).create(out).unwrap();

    for img in pics {
        let img = img.unwrap();
        if !img.file_type().unwrap().is_file() {
            continue;
        }
        let p = img.path();
        let stem = p.file_stem().unwrap();
        let path = img.path();
        let img = ImageReader::new(Cursor::new(fs::read(path).unwrap()))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let size = 100;
        let img = img.resize(size, size, FilterType::Lanczos3);
        let new_name = format!("{}.png", stem.to_str().unwrap());
        let store = {
            let mut s = out.to_owned();
            s.push(new_name);
            s
        };
        img.save(store).unwrap();
    }
    // let img = img.resize(120, 80, FilterType::Lanczos3);

    // let last_map = gen_map(&img.to_rgb8()).0;
    // let mut img = img.to_rgb8();
    // for x in 0..img.width() {
    //     for y in 0..img.height() {
    //         let pix = img.get_pixel(x, y);
    //         img.put_pixel(x, y, last_map[nearest(last_map, pix)]);
    //     }
    // }
    // img.save("out.png").unwrap();
}

// #[test]
// fn a() {
//     let rgb: Rgb = Rgb::from([0., 0., 0.5]);
//     let hsv: Hsv = Hsv::from_color(rgb);
//     dbg!(hsv.get_hue().into_inner());
// }

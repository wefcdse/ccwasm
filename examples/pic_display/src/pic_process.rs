use std::{collections::HashMap, ops::AddAssign};

use image::{GenericImageView, Pixel, Rgb};
use palette::{FromColor, GetHue, Hsv};
const L: usize = 16;

pub fn nearest(map: [Rgb<u8>; L], pix: &Rgb<u8>) -> usize {
    let mut min_dist2 = i64::MAX;
    let mut min_idx = 0usize;
    let [a, b, c] = pix.0;
    let [a, b, c] = [a as i64, b as i64, c as i64];

    for i in 0..L {
        let [a0, b0, c0] = map[i].0;
        let [a0, b0, c0] = [a0 as i64, b0 as i64, c0 as i64];

        // let dist2 = (a0 - a) * (a0 - a) + (b0 - b) * (b0 - b) + (c0 - c) * (c0 - c);
        let dist2 = cacl_dist([a0, b0, c0], [a, b, c]);
        if dist2 < min_dist2 {
            min_dist2 = dist2;
            min_idx = i;
        }
    }
    min_idx
}

pub fn gen_map(img: &impl GenericImageView<Pixel = Rgb<u8>>) -> [Rgb<u8>; L] {
    let mut hs: HashMap<_, usize> = HashMap::new();
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pix = img.get_pixel(x, y).to_rgb();
            hs.entry(pix).or_default().add_assign(1);
        }
    }
    // dbg!(hs.len(), img.width() * img.height());
    // // return;
    // dbg!(hs.values().max());

    let pix_map = {
        let mut m = [Rgb([0u8, 0, 0]); L];
        for (a, b) in m.iter_mut().zip(hs.keys()) {
            *a = *b;
        }
        m
    };
    let mut epochs = 0;
    const MAX_EPOCH: usize = 200;
    let mut last_map = pix_map;
    loop {
        if epochs >= MAX_EPOCH {
            break;
        }
        epochs += 1;
        let mut count_map: [([i64; 3], usize); L] = [([0, 0, 0], 0); L];

        for (pix, count) in hs.iter() {
            assert_ne!(*count, 0);
            let [a, b, c] = pix.0;
            let [a, b, c] = [a as i64, b as i64, c as i64];

            let mut min_dist2 = i64::MAX;
            let mut min_idx = 0usize;

            for i in 0..L {
                let [a0, b0, c0] = last_map[i].0;
                let [a0, b0, c0] = [a0 as i64, b0 as i64, c0 as i64];

                // let dist2 = (a0 - a) * (a0 - a) + (b0 - b) * (b0 - b) + (c0 - c) * (c0 - c);
                let dist2 = cacl_dist([a0, b0, c0], [a, b, c]);
                if dist2 < min_dist2 {
                    min_dist2 = dist2;
                    min_idx = i;
                }
            }
            count_map[min_idx].0[0] += a * *count as i64;
            count_map[min_idx].0[1] += b * *count as i64;
            count_map[min_idx].0[2] += c * *count as i64;
            count_map[min_idx].1 += *count;
        }

        let new_pix_map = {
            let mut m = [Rgb([0u8, 0, 0]); L];
            for (([a, b, c], count), p) in count_map.iter().zip(m.iter_mut()) {
                if *count == 0 {
                    continue;
                }
                assert_ne!(*count, 0);
                let a = (*a / *count as i64) as u8;
                let b = (*b / *count as i64) as u8;
                let c = (*c / *count as i64) as u8;
                *p = Rgb([a, b, c]);
            }
            m
        };
        if new_pix_map == last_map {
            break;
        }
        last_map = new_pix_map;
    }
    last_map
}
fn cacl_dist(a: [i64; 3], b: [i64; 3]) -> i64 {
    let higha = if a[0] > a[1] && a[0] > a[2] {
        0
    } else if a[1] > a[2] {
        1
    } else {
        2
    };
    let highb = if b[0] > b[1] && b[0] > b[2] {
        0
    } else if b[1] > b[2] {
        1
    } else {
        2
    };
    let oolor_fix = if higha == highb { 0 } else { 500 };

    (a[0] - b[0]) * (a[0] - b[0])
        + (a[1] - b[1]) * (a[1] - b[1])
        + (a[2] - b[2]) * (a[2] - b[2])
        + oolor_fix
}

#[allow(dead_code)]
fn cacl_dist2(a: [i64; 3], b: [i64; 3]) -> i64 {
    let a = {
        let a1 = [a[0] as f32 / 255., a[1] as f32 / 255., a[2] as f32 / 255.];
        let c: palette::rgb::Rgb = palette::rgb::Rgb::from(a1);
        let d = Hsv::from_color(c);
        d
    };
    let b = {
        let a1 = [b[0] as f32 / 255., b[1] as f32 / 255., b[2] as f32 / 255.];
        let c: palette::rgb::Rgb = palette::rgb::Rgb::from(a1);
        let d = Hsv::from_color(c);
        d
    };
    let ha: f32 = a.get_hue().into();
    let hb: f32 = b.get_hue().into();
    let difh = {
        let hd = (ha - hb).abs();
        let hd = if hd > 180. { hd - 180. } else { hd };
        hd / 180.
    };
    let difv = { (a.value - b.value).abs() };
    let difs = { (a.saturation - b.saturation).abs() };

    let target = difh.square() * 10. + difv.square() + difs.square();
    (target * 10000.) as i64
}
trait Sqr {
    fn square(self) -> Self;
}
impl Sqr for i64 {
    fn square(self) -> Self {
        self * self
    }
}
impl Sqr for f32 {
    fn square(self) -> Self {
        self * self
    }
}

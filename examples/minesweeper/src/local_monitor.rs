use std::time::Duration;

use crate::{
    functions,
    utils::{AsIfPixel, ColorId, Direction},
    vec2d::Vec2d,
};

/// a monitor but stores the pixel localy,
/// and can send only changed pixels
///
/// x, y starts with 1
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalMonitor {
    data: Vec2d<AsIfPixel>,
    changed: Vec2d<bool>,
    wait_time: Duration,
    wait_count: usize,
}

// creating
impl LocalMonitor {
    pub fn new(x: usize, y: usize, pixel: AsIfPixel) -> Self {
        Self {
            data: Vec2d::new_filled_copy(x, y, pixel),
            changed: Vec2d::new_filled_copy(x, y, true),
            wait_time: Duration::from_secs_f32(0.05),
            wait_count: 75,
        }
    }
    pub fn resize(&mut self, x: usize, y: usize, pixel: AsIfPixel) {
        self.data = Vec2d::new_filled_copy(x, y, pixel);
        self.changed = Vec2d::new_filled_copy(x, y, true);
    }
    pub fn size(&self) -> (usize, usize) {
        self.data.size()
    }
    pub fn x(&self) -> usize {
        self.data.x()
    }
    pub fn y(&self) -> usize {
        self.data.y()
    }
}

// useing
impl LocalMonitor {
    /// x, y starts with 1
    pub fn get(&self, x: usize, y: usize) -> Option<AsIfPixel> {
        if x > self.x() || y > self.y() {
            None
        } else {
            let x = x - 1;
            let y = y - 1;
            Some(self.data[(x, y)])
        }
    }
    /// x, y starts with 1
    pub fn write(&mut self, x: usize, y: usize, pixel: AsIfPixel) {
        if x > self.x() || y > self.y() || x == 0 || y == 0 {
            return;
        }
        let x = x - 1;
        let y = y - 1;
        let p0 = self.data[(x, y)];
        if p0 != pixel {
            self.data[(x, y)] = pixel;
            self.changed[(x, y)] = true;
        }
    }

    pub fn clear_with(&mut self, color: ColorId) {
        for x in 1..=self.x() {
            for y in 1..=self.y() {
                let pixel = AsIfPixel::colored_whitespace(color);
                self.write(x, y, pixel);
            }
        }
    }

    /// write a [str], ignore non-ASCII chars
    pub fn write_str(
        &mut self,
        x: usize,
        y: usize,
        direction: Direction,
        text: &str,
        background_color: ColorId,
        text_color: ColorId,
    ) {
        let (dx, dy) = direction.to_dxdy();
        let (size_x, size_y) = self.size();
        let (size_x, size_y) = (size_x as isize, size_y as isize);

        let mut now_x = x as isize;
        let mut now_y = y as isize;
        #[allow(unused)]
        let (x, y) = ((), ());

        for c in text.chars() {
            let pixel = if let Some(p) = AsIfPixel::new(c, background_color, text_color) {
                p
            } else {
                continue;
            };
            self.write(now_x as usize, now_y as usize, pixel);
            now_x += dx;
            now_y += dy;
            if now_x <= 0 || now_x > size_x || now_y <= 0 || now_y > size_y {
                return;
            }
        }
    }
}

impl LocalMonitor {
    pub async fn sync(&mut self) -> usize {
        let mut count = 0;

        for ((x, y), changed) in self.changed.iter() {
            if *changed {
                let pixel = self.data[(x, y)];

                functions::write_pix(x as i32, y as i32, pixel).await;

                count += 1;
            }
        }

        self.changed = Vec2d::new_filled_copy(self.x(), self.y(), false);
        count
    }

    pub async fn sync_all(&mut self) {
        for ((x, y), pixel) in self.data.iter() {
            functions::write_pix(x as i32, y as i32, *pixel).await;
        }
        self.changed = Vec2d::new_filled_copy(self.x(), self.y(), false);
    }
}

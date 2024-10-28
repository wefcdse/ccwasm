use std::io::Cursor;

use cc_wasm_api::{
    addon::{
        arg::get_args,
        local_monitor::LocalMonitor,
        misc::{AsIfPixel, ColorId, Side},
    },
    export_funcs,
    prelude::CoroutineSpawn,
    throw, throw_exec,
};
use image::{imageops::FilterType, ImageReader};
use pic_process::{gen_map, nearest};
export_funcs!(init);

fn init() {
    // TickSyncer::spawn_handle_coroutine();
    async {
        // let mut ts = TickSyncer::new();

            

        let mut m = throw!(LocalMonitor::new_inited(Side::Top).await);
        // ts.sync().await;
        let file: Vec<u8> = throw!(get_args().await);

        let img =
            throw!(throw!(ImageReader::new(Cursor::new(&file)).with_guessed_format()).decode());
        // ts.sync().await;

        // let img = img.resize(m.x() as u32, m.y() as u32, FilterType::Gaussian);
        let img = img.to_rgb8();
        let (x, y) = {
            let ix = img.width();
            let iy = img.height();
            let ixy_rate = ix as f32 / iy as f32;
            let xy_rate = LocalMonitor::xy_rate();

            let ry = (m.x() as f32 / ixy_rate / xy_rate) as u32;
            let rx = (m.y() as f32 * ixy_rate * xy_rate) as u32;
            if ry <= m.y() as u32 {
                (m.x() as u32, ry)
            } else {
                (rx, m.y() as u32)
            }
        };

        let img = image::imageops::resize(&img, x, y, FilterType::Gaussian);
        // ts.sync().await;

        let last_map = gen_map(&img);
        // ts.sync().await;

        for (idx, clr) in last_map.iter().enumerate() {
            let target = clr.0;
            let target = target[0] as u32 * 256 * 256 + target[1] as u32 * 256 + target[2] as u32;
            throw!(
                m.set_palette(ColorId::from_number_overflow(idx as u32), target)
                    .await
            );
        }
        // ts.sync().await;

        for x in 0..img.width() {
            for y in 0..img.height() {
                let pix = img.get_pixel(x, y);
                m.write(
                    x as usize + 1,
                    y as usize + 1,
                    AsIfPixel::colored_whitespace(ColorId::from_number_overflow(nearest(
                        last_map, pix,
                    )
                        as u32)),
                );
            }
        }
        // ts.sync().await;

        throw!(m.sync().await);

        // ts.sync().await;
    }
    .spawn();
}

pub mod pic_process;

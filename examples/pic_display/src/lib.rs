use cc_wasm_api::{
    addon::{
        arg::get_args,
        local_monitor::LocalMonitor,
        misc::{AsIfPixel, ColorId, Side},
    },
    export_funcs,
    prelude::{CoroutineSpawn, TickSyncer},
    throw, throw_exec,
};
use image::{imageops::FilterType, ImageReader};
use pic_process::{gen_map, nearest};
use std::io::Cursor;
export_funcs!(init);
macro_rules! time {
    ($st:ident) => {
        let $st = ::std::time::Instant::now();
    };
    ($st:ident, $info:literal) => {
        throw_exec!(&format!(
            "print({:?})",
            format!("{}:{:?}", $info, $st.elapsed())
        ));
    };
}

fn init() {
    TickSyncer::spawn_handle_coroutine();
    async {
        let mut ts = TickSyncer::new();
        let mut m = throw!(LocalMonitor::new_inited(Side::Top).await);
        let file: Vec<u8> = throw!(get_args().await);

        time!(s1);
        let img =
            throw!(throw!(ImageReader::new(Cursor::new(&file)).with_guessed_format()).decode());
        time!(s1, "img load");

        ts.sync().await;

        time!(s2);
        let img = {
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
            ts.sync().await;

            let img = image::imageops::resize(&img, x, y, FilterType::Gaussian);
            img
        };
        time!(s2, "img resize");
        ts.sync().await;

        time!(s3);
        let last_map = gen_map(&img);
        time!(s3, "pla gen");

        for (idx, clr) in last_map.iter().enumerate() {
            let target = clr.0;
            let target = target[0] as u32 * 256 * 256 + target[1] as u32 * 256 + target[2] as u32;
            throw!(
                m.set_palette(ColorId::from_number_overflow(idx as u32), target)
                    .await
            );
        }

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

        time!(s4);
        throw!(m.sync().await);
        time!(s4, "img draw");
        time!(s1, "total");
        cc_wasm_api::coroutine::stop();
    }
    .spawn();
}

pub mod pic_process;

use std::time::Duration;

use cc_wasm_api::{
    addon::{
        local_monitor::LocalMonitor,
        misc::{AsIfPixel, ColorId, Side},
    },
    eval::exec,
    export_funcs,
    prelude::{CoroutineSpawn, TickSyncer},
};
use rusttype::{Font, Point, Scale};
use stupid_utils::select::DotSelect;

export_funcs!(init);

fn init() {
    TickSyncer::spawn_handle_coroutine();
    async {
        let mut ts = TickSyncer::new();
        let mut m = LocalMonitor::new_inited(Side::Top).await.unwrap();

        exec("print(\"hell world\")").await.unwrap();
        exec(&format!("print(\"{}, {}\")", m.x(), m.y()))
            .await
            .unwrap();

        let font_data: &[u8] = include_bytes!("fonts/CALIBRI.ttf");
        let font_data: &[u8] = include_bytes!("fonts/SIMYOU.ttf");
        let font = Font::try_from_bytes(font_data).unwrap();

        let font_x = from_y_to_x(m.y());
        let font_y = m.y();
        loop {
            for c in include_str!("txt.txt").chars() {
                // let c = '，';
                m.clear(ColorId::Black).await.unwrap();
                // m.clear_local(ColorId::Black);
                let scaled = font.glyph(c).scaled(Scale {
                    x: font_x as f32 * (54. / 50.),
                    y: font_y as f32 * (35. / 33.),
                    // x: m.x() as f32,
                    // y: m.y() as f32,
                });
                // let max = Point {
                //     x: m.x() as f32,
                //     y: m.y() as f32,
                // };
                // let target = max - b.max;
                let h = scaled.h_metrics();

                let positioned = scaled.positioned(Point {
                    x: 0.,
                    y: font_y as f32 * 0.89,
                });
                let b = positioned.pixel_bounding_box().unwrap();
                exec(&format!("print(\"{}, {}\")", m.x(), m.y()))
                    .await
                    .unwrap();

                positioned.draw(|x, y, v| {
                    let x = x + b.min.x as u32;
                    let y = y + b.min.y as u32;
                    let color = (v > 0.3).select(ColorId::White, ColorId::Black);
                    m.write(x as usize, y as usize, AsIfPixel::colored_whitespace(color));
                });
                m.sync().await.unwrap();
                ts.sleep(Duration::from_secs_f32(0.005)).await;
                ts.sync().await;
            }
        }
    }
    .spawn();
}
fn from_y_to_x(y: usize) -> usize {
    match y {
        5 => 7,
        12 => 18,
        19 => 29,
        26 => 39,
        33 => 50,
        40 => 61,
        _ => panic!(),
    }
}

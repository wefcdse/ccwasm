use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use cc_wasm_api::{
    addon::{
        local_monitor::LocalMonitor,
        misc::{AsIfPixel, ColorId, Side},
    },
    eval::{eval, exec},
    export_funcs,
    prelude::{CoroutineSpawn, TickSyncer},
};
use rusttype::{Font, Point, Scale};
use stupid_utils::{disable, select::DotSelect};

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

        let txt: String = eval("return global.args[1]").await.unwrap();
        let txt = String::from_utf8(STANDARD.decode(txt).unwrap()).unwrap();
        // let txt = include_str!("txt.txt");

        // let font_data: &[u8] = include_bytes!("fonts/CALIBRI.ttf");
        let font_data: &[u8] = include_bytes!("fonts/江城知音体 600W");
        let font = Font::try_from_bytes(font_data).unwrap();

        let font_x = from_y_to_x(m.y());
        let font_y = m.y();

        let draw_cache = buffed_draw(&txt, &font, font_x, font_y);
        disable!(font);

        let mut basic_offs: i32 = m.x() as i32 + 2;
        loop {
            // m.clear(ColorId::Black).await.unwrap();
            m.clear_local(ColorId::Black);
            let now = Instant::now();
            let mut offset_x: i32 = basic_offs;
            let mut valid = false;
            for c in txt.chars() {
                // let c = '，';
                // m.clear_local(ColorId::Black);
                let draw = draw_cache.get(&c).unwrap();

                // exec(&format!("print(\"{}, {}\")", m.x(), m.y()))
                //     .await
                //     .unwrap();
                if offset_x > m.x().try_into().unwrap() {
                    valid = true;
                    break;
                }
                if offset_x + draw.width <= 0 {
                    offset_x += draw.width + from_y_to_x_offs(font_y);
                    continue;
                }

                draw.pic.iter().for_each(|(x, y, v)| {
                    let x = offset_x + *x as i32;
                    let y = *y as i32;
                    if x >= 0 {
                        let color = v.select(ColorId::Purple, ColorId::Black);
                        m.write(x as usize, y as usize, AsIfPixel::colored_whitespace(color));
                    }
                });
                offset_x += draw.width + from_y_to_x_offs(font_y);
                valid = true;
            }

            exec(&format!("print({})", now.elapsed().as_secs_f32()))
                .await
                .unwrap();
            let now = Instant::now();
            m.sync().await.unwrap();
            exec(&format!("print({})", now.elapsed().as_secs_f32()))
                .await
                .unwrap();
            basic_offs -= from_y_to_x_offs(font_y);
            if !valid {
                basic_offs = m.x() as i32 + 2;
            }
            // ts.sleep(Duration::from_secs_f32(0.005)).await;
            ts.sync().await;
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
fn from_y_to_x_offs(y: usize) -> i32 {
    match y {
        5 => 1,
        12 => 1,
        19 => 2,
        26 => 3,
        33 => 3,
        40 => 4,
        _ => panic!(),
    }
}

struct BufDraw {
    width: i32,
    pic: Vec<(u32, u32, bool)>,
}

fn buffed_draw(s: &str, font: &Font, font_x: usize, font_y: usize) -> HashMap<char, BufDraw> {
    let chars = s.chars().collect::<HashSet<_>>();

    let o = chars
        .into_iter()
        .map(|c| {
            if c.is_whitespace() {
                return (
                    c,
                    BufDraw {
                        width: (font_x as f32 * (56. / 50.) / 2.) as i32,
                        pic: Vec::new(),
                    },
                );
            }

            let scaled = font.glyph(c).scaled(Scale {
                x: font_x as f32 * (56. / 50.),
                y: font_y as f32 * (37. / 33.),
            });

            let positioned = scaled.positioned(Point {
                x: 0.,
                y: font_y as f32 * 0.97,
            });
            let b = positioned.pixel_bounding_box().unwrap();

            let mut pic = Vec::new();
            positioned.draw(|x, y, v| {
                let x = x as i32 + b.min.x as i32;
                let y = y + b.min.y as u32;

                let color = v > 0.5;
                pic.push((x as u32, y as u32, color));
            });

            (
                c,
                BufDraw {
                    width: b.width(),
                    pic,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    o
}

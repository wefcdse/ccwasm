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
    debug::show_str,
    eval::{eval, exec},
    export_funcs,
    prelude::{CoroutineSpawn, TickSyncer},
    throw, throw_eval, throw_exec,
    utils::Number,
};
use rusttype::{Font, Point, Scale};
use stupid_utils::{disable, prelude::MutableInit, select::DotSelect};
const SIDE: Side = Side::Top;

export_funcs!(init);
fn init() {
    TickSyncer::spawn_handle_coroutine();
    async {
        let mut ts = TickSyncer::new();
        // let mut m = LocalMonitor::new_inited(Side::Top).await.unwrap();

        throw_exec!("print(\"hell world\")");
        // throw_exec!(&format!("print(\"{}, {}\")", m.x(), m.y()));

        let txt = {
            // let txt = txt
            //     .map(|txt| String::from_utf8(STANDARD.decode(txt).unwrap()).unwrap())
            //     .unwrap_or(include_str!("txt.txt").to_owned());
            // txt
            match throw_eval!(Option<String>, "return global.args[1]") {
                Some(s) => throw!(String::from_utf8(throw!(STANDARD.decode(s)))),
                None => include_str!("txt.txt").to_owned(),
            }
        };
        // let txt = include_str!("txt.txt");
        let (mut monitors, len_x) = {
            let monitor_names = throw_eval!(Vec<String>, "return unpack(global.args)")
                .mutable_init(|v| {
                    if !v.is_empty() {
                        v.remove(0);
                    }
                });

            let mut monitors = Vec::new();
            for name in monitor_names {
                monitors.push(throw!(LocalMonitor::new_inited((SIDE, &name)).await));
            }
            if monitors.is_empty() {
                throw!("at least 1 monitor");
            }

            {
                let w = monitors[0].y();
                for m in &monitors {
                    if w != m.y() {
                        throw!("monitors must be has same y");
                    }
                }
            }
            let mut s = 0;
            (
                monitors
                    .into_iter()
                    .map(|m| {
                        let o = (m, s);
                        s += o.0.x();
                        o
                    })
                    .collect::<Vec<_>>(),
                Number::Int(s as i64),
            )
        };

        // let start_x = throw_eval!(Option<String>, "return global.args[3]")
        //     .map(|s| Number::Float(s.parse::<f64>().unwrap()))
        //     .unwrap_or(Number::Int(0));
        // let len_x = throw_eval!(Option<String>, "return global.args[2]")
        //     .map(|s| Number::Float(s.parse::<f64>().unwrap()))
        //     .unwrap_or(Number::Int(m.x() as i64));

        // let start_x = Number::Int(82);
        // let len_x = Number::Int(82 * 2);

        // let font_data: &[u8] = include_bytes!("fonts/CALIBRI.ttf");
        // let font_data: &[u8] = include_bytes!("fonts/STXINGKA.TTF");
        // let font_data: &[u8] = include_bytes!("fonts/江城知音体 600W");
        let font_data: &[u8] = include_bytes!("fonts/SIMYOU.TTF");
        let font = Font::try_from_bytes(font_data).unwrap();

        let font_x = from_y_to_x(monitors[0].0.y());
        let font_y = monitors[0].0.y();

        let draw_cache = buffed_draw(&txt, &font, font_x, font_y);
        disable!(font);

        let base_offs_init: i32 = len_x.to_i32() + 2;

        let mut basic_offs: i32 = base_offs_init;
        loop {
            let start = Instant::now();
            // m.clear(ColorId::Black).await.unwrap();
            monitors.iter_mut().for_each(|(m, _offs)| {
                m.clear_local(ColorId::Black);
            });

            // let now = Instant::now();
            let mut offset_x: i32 = basic_offs;
            let mut valid = false;
            for c in txt.chars() {
                // let c = '，';
                // m.clear_local(ColorId::Black);
                let draw = draw_cache.get(&c).unwrap();

                // exec(&format!("print(\"{}, {}\")", m.x(), m.y()))
                //     .await
                //     .unwrap();
                if offset_x > len_x.to_i32() {
                    valid = true;
                    break;
                }
                if offset_x + draw.width <= 0 {
                    offset_x += draw.width + from_y_to_x_offs(font_y);
                    continue;
                }

                draw.pic.iter().for_each(|(x, y, v)| {
                    for (m, offs) in monitors.iter_mut() {
                        let x = offset_x - *offs as i32 + *x as i32;
                        let y = *y as i32;
                        if x >= 0 {
                            let color = v.select(ColorId::Purple, ColorId::Black);
                            m.write(x as usize, y as usize, AsIfPixel::colored_whitespace(color));
                        }
                    }
                });
                offset_x += draw.width + from_y_to_x_offs(font_y);
                valid = true;
            }

            // exec(&format!("print({})", now.elapsed().as_secs_f32()))
            //     .await
            //     .unwrap();
            // let now = Instant::now();

            // let rs: bool = eval("return redstone.getInput(\"back\")").await.unwrap();
            // if !last_rs && rs {
            basic_offs -= from_y_to_x_offs(font_y);
            // }
            // last_rs = rs;
            if !valid {
                basic_offs = base_offs_init;
            }
            show_str(&format!(
                "draw time: {}ms",
                start.elapsed().as_secs_f32() * 1000.
            ));

            // ts.sleep(Duration::from_secs_f32(0.005)).await;
            let start = Instant::now();
            let (script, code_line) = {
                let mut so = String::new();
                let mut co = 0;
                for (m, _) in monitors.iter_mut() {
                    let (s, c) = unsafe { m.sync_clear_script(ColorId::Black) };
                    so += &s;
                    co += c;
                }
                (so, co)
            };
            show_str(&format!(
                "cacl time: {}ms",
                start.elapsed().as_secs_f32() * 1000.
            ));

            let start = Instant::now();
            // show_str(&script);

            throw_exec!(&script);
            show_str(&format!(
                "exec time: {}ms",
                start.elapsed().as_secs_f32() * 1000.
            ));

            show_str(&format!("code line: {}", code_line));

            ts.sync().await;
            // m.sync_limited_rate(0.4).await.unwrap();
            // ts.sync().await;
            // m.sync_limited_rate(0.3).await.unwrap();
            // ts.sync().await;
            // m.sync_limited(120).await.unwrap();
            // ts.sync().await;
            // m.sync_limited(120).await.unwrap();
            // ts.sync().await;
            // ts.sync().await;
            // ts.sync().await;
            // ts.sync().await;
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
        _ => 2,
        5 => 2,
        12 => 2,
        19 => 2,
        26 => 2,
        33 => 2,
        40 => 2,
        _ => panic!(),
    }
}

struct BufDraw {
    width: i32,
    pic: Vec<(u32, u32, bool)>,
}

fn buffed_draw(s: &str, font: &Font, font_x: usize, font_y: usize) -> HashMap<char, BufDraw> {
    // let (w, h, hi, vh) = (56., 37., 0.97, 0.4);
    let (w, h, hi, vh) = (54., 35., 0.92, 0.25);

    let chars = s.chars().collect::<HashSet<_>>();

    let o = chars
        .into_iter()
        .map(|c| {
            if c.is_whitespace() {
                return (
                    c,
                    BufDraw {
                        width: (font_x as f32 * (w / 50.) / 2.) as i32,
                        pic: Vec::new(),
                    },
                );
            }

            let scaled = font.glyph(c).scaled(Scale {
                x: font_x as f32 * (w / 50.),
                y: font_y as f32 * (h / 33.),
            });

            let positioned = scaled.positioned(Point {
                x: 0.,
                y: font_y as f32 * hi,
            });
            let b = positioned.pixel_bounding_box().unwrap();

            let mut pic = Vec::new();
            positioned.draw(|x, y, v| {
                let x = x as i32 + b.min.x as i32;
                let y = y + b.min.y as u32;

                let color = v > vh;
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

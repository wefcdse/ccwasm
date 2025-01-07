pub mod pic_process;
use cc_wasm_api::{
    addon::{
        arg::get_args,
        local_monitor::LocalMonitor,
        misc::{AsIfPixel, ColorId, Side},
    },
    export_funcs,
    prelude::{CoroutineSpawn, LuaResult, TickSyncer},
    throw, throw_eval, throw_exec, time,
};
use image::{imageops::FilterType, ImageReader};
use pic_process::{gen_map, nearest};
use std::io::Cursor;
export_funcs!(init);

fn init() {
    TickSyncer::spawn_handle_coroutine();
    async {
        let mut ts = TickSyncer::new();
        throw_exec!("print(151)");
        let mut m = throw!(LocalMonitor::new_inited(Side::Top).await);
        let file: String = throw!(get_args().await);

        let mut script = String::new();
        throw!(gen_drawscript(&mut script, &mut ts, &file,Q &mut m).await);

        throw_exec!(&script);
        cc_wasm_api::coroutine::stop();
    }
    .spawn();
}

async fn gen_drawscript(
    script: &mut String,
    ts: &mut TickSyncer,
    pic: &str,
    monitor: &mut LocalMonitor,
) -> LuaResult<usize> {
    let mut code = 0;

    let file: Vec<u8> = {
        let script = format!(r#"return fs.open({:?}, "r").readAll()"\#"#, pic);
        throw_eval!(&script)
    };
    ts.sync().await;

    let img = ImageReader::new(Cursor::new(&file))
        .with_guessed_format()?
        .decode()?;
    ts.sync().await;

    let img = {
        let img = img.to_rgb8();
        let (x, y) = {
            let ix = img.width();
            let iy = img.height();
            let ixy_rate = ix as f32 / iy as f32;
            let xy_rate = LocalMonitor::xy_rate();

            let ry = (monitor.x() as f32 / ixy_rate / xy_rate) as u32;
            let rx = (monitor.y() as f32 * ixy_rate * xy_rate) as u32;
            if ry <= monitor.y() as u32 {
                (monitor.x() as u32, ry)
            } else {
                (rx, monitor.y() as u32)
            }
        };
        ts.sync().await;

        let img = image::imageops::resize(&img, x, y, FilterType::Gaussian);
        img
    };
    ts.sync().await;

    let (last_map, bg_color_index) = gen_map(&img);
    ts.sync().await;

    for (idx, clr) in last_map.iter().enumerate() {
        let target = clr.0;
        let target = target[0] as u32 * 256 * 256 + target[1] as u32 * 256 + target[2] as u32;
        code +=
            monitor.set_palette_script(script, ColorId::from_number_overflow(idx as u32), target);
    }
    ts.sync().await;

    code += unsafe {
        monitor.clear_script(script, ColorId::from_number_overflow(bg_color_index as u32))
    };
    ts.sync().await;

    for x in 0..img.width() {
        for y in 0..img.height() {
            let pix = img.get_pixel(x, y);
            monitor.write(
                x as usize + 1,
                y as usize + 1,
                AsIfPixel::colored_whitespace(ColorId::from_number_overflow(
                    nearest(last_map, pix) as u32,
                )),
            );
        }
    }
    ts.sync().await;

    code += unsafe { monitor.sync_script(script) };
    Ok(code)
}

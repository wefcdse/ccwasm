use cc_wasm_api::prelude::*;

// use crate::utils::AsIfPixel;

// pub async fn write_pix(x: i32, y: i32, pix: AsIfPixel) {
//     // return;
//     let script = format!(
//         "global.monitor.setCursorPos({x}, {y})
//     global.monitor.setBackgroundColour({bc})
//     global.monitor.setTextColour({tc})
//     global.monitor.write(\"{txt}\")",
//         x = x + 1,
//         y = y + 1,
//         bc = pix.background_color.to_number(),
//         tc = pix.text_color.to_number(),
//         txt = pix.text()
//     );
//     // show_str(&script);
//     exec(&script).await.unwrap();
// }

// pub async fn init_monitor() {
//     exec("global.monitor = peripheral.wrap(\"top\")")
//         .await
//         .unwrap();
// }

pub async fn poll_evt() {
    let script: &str = r#"
        os.queueEvent("aaaa")
        local event_data = {os.pullEvent()}
        if event_data[1] == "monitor_touch" then
            return unpack(event_data)
        end
        if event_data[1] == "aaaa" then
            return
        end
        print(event_data[1])
        "#;
    for _ in 0..20 {
        if let Some((_, _, x, y)) = eval::<Option<(String, String, Number, Number)>>(script)
            .await
            .unwrap()
        {
            crate::CLICKED.insert((x.to_i32(), y.to_i32())).await;
        }
    }
}

// pub async fn get_side() {
//     let script: &str = r#"
//         os.queueEvent("aaaa")
//         local event_data = {os.pullEvent()}
//         if event_data[1] == "monitor_touch" then
//             return unpack(event_data)
//         end
//         if event_data[1] == "aaaa" then
//             return
//         end
//         print(event_data[1])
//         "#;
//     for _ in 0..20 {
//         if let Some((_, _, x, y)) = eval::<Option<(String, String, Number, Number)>>(script)
//             .await
//             .unwrap()
//         {
//             crate::CLICKED.insert((x.to_i32(), y.to_i32())).await;
//         }
//     }
// }

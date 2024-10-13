use cc_wasm_api::{
    addon::{
        functions,
        local_monitor::LocalMonitor,
        misc::{ColorId, Side},
    },
    debug::show_str,
    prelude::*,
};
use std::time::{Duration, Instant};
export_funcs!(init, aa);

fn aa(a: Either<Number, Nil>, b: String) -> (bool, Either<String, Nil>, i32) {
    (true, Either::First(format!("{:?}, {}", a, b)), 3)
}

fn init() {
    TickSyncer::spawn_handle_coroutine();
    async {
        let mut ts = TickSyncer::new();
        functions::init_monitor(Side::Back).await.unwrap();
        show_str("aaaaaa");
        let mut monitor = LocalMonitor::init(Side::Back).await.unwrap();
        show_str(&format!("{:?}", monitor));
        monitor.clear(ColorId::Blue).await.unwrap();
        loop {
            monitor.clear(ColorId::Blue).await.unwrap();
            ts.sync().await;
            monitor.clear(ColorId::Gray).await.unwrap();
            ts.sync().await;
        }
    }
    .spawn();
}

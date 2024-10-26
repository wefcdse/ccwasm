use cc_wasm_api::{
    addon::{
        local_monitor::LocalMonitor,
        misc::{ColorId, Side},
        throw::Throw,
    },
    debug::{show_debug_desc, show_str},
    info,
    lua_api::LuaError,
    prelude::*,
    throw, throw_eval, throw_exec,
};
use std::{
    env,
    error::Error,
    fmt::Display,
    time::{Duration, Instant},
};
export_funcs!(init, aa);

fn aa(a: Either<Number, Nil>, b: String) -> (bool, Either<String, Nil>, i32) {
    (true, Either::First(format!("{:?}, {}", a, b)), 3)
}
#[derive(Debug)]
struct E1;
impl Display for E1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error1")
    }
}
impl Error for E1 {}

fn init() {
    TickSyncer::spawn_handle_coroutine();
    async {
        let mut ts = TickSyncer::new();
        throw_exec!("print(32)");
        let a = throw_eval!("return 2", Number);
        show_str("aaaaaa");
        let mut monitor = throw!(LocalMonitor::new_inited(Side::Top).await);
        throw!(monitor.clear(ColorId::Blue).await);
        loop {
            monitor.clear_local(ColorId::Blue);
            show_str("c1");
            let c = throw!(monitor.sync().await);
            show_debug_desc("description", &c);
            show_str("s1");
            ts.sync().await;

            monitor.clear_local(ColorId::Gray);
            show_str("c2");
            throw!(monitor.sync().await);
            show_str("s2");
            ts.sync().await;
        }
    }
    .spawn();
}

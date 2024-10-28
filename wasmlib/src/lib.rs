use cc_wasm_api::{addon::arg, debug::show_str, prelude::*};

export_funcs!(init);

fn init() {
    async {
        let args: Vec<u8> = arg::get_args().await.unwrap();
        let a = String::from_utf8(args).unwrap();
        show_str(&a);
    }
    .spawn();
}

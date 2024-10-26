use ::core::str;
use std::{io::Write, sync::Arc, time::Instant};

use anyhow::{anyhow, Result};
use wasmi::*;
#[no_mangle]
pub extern "C" fn main() {
    println!("{}", 32);
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();

    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.
    let module = Module::new(&engine, include_bytes!("../../a.wasm")).unwrap();

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    let mut store = Store::new(&engine, "String::new()".to_owned());

    let s = Arc::new("a + 1");

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = Linker::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let hello = instance.get_typed_func::<(), ()>(&store, "call").unwrap();

    // And finally we can call the wasm!
    hello.call(&mut store, ()).unwrap();
    hello.call(&mut store, ()).unwrap();
    hello.call(&mut store, ()).unwrap();
    let now = Instant::now();
    hello.call(&mut store, ()).unwrap();
    dbg!(now.elapsed().as_secs_f64() * 1000_000_000. / 10_000_000.);
}

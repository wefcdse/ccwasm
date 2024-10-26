use ::core::str;
use std::{
    hint::black_box,
    io::Write,
    mem::transmute,
    ops::AddAssign,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Result};
use wasmi::*;
fn main() -> Result<()> {
    dbg!(b(1_000_000_000));
    Ok(())
}

fn masin() -> Result<()> {
    dbg!(b(1000_000_000));
    // dbg!(Instant::now());
    println!("qwfwf");
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();

    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.
    let module = Module::new(&engine, include_bytes!("../../a.wasm"))?;

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    let mut store = Store::new(&engine, "String::new()".to_owned());

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = Linker::new(&engine);
    wasmipython::link(&mut store, &mut linker).unwrap();

    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    let hello = instance.get_typed_func::<(), f64>(&store, "call")?;

    // And finally we can call the wasm!
    hello.call(&mut store, ())?;
    hello.call(&mut store, ())?;
    hello.call(&mut store, ())?;
    let now = Instant::now();
    dbg!(hello.call(&mut store, ())?);
    dbg!(hello.call(&mut store, ())?);
    dbg!(hello.call(&mut store, ())?);
    dbg!(hello.call(&mut store, ())?);
    dbg!(now.elapsed().as_secs_f64() * 1000_000_000. / 10_000_000.);
    Ok(())
}
fn b(c: usize) -> f64 {
    let ts = Instant::now();
    let mut num = black_box((black_box(32)));
    for _ in 0..c {
        num.add_assign(black_box(1));
    }
    black_box(num);
    let d = unsafe { transmute::<Instant, Duration>(ts) };
    return ts.elapsed().as_secs_f64() * 1000_000_000. / c as f64;
    return d.as_secs_f64();
}

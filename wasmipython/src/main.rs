use ::core::str;
use std::{io::Write, sync::Arc};

use anyhow::{anyhow, Result};
use wasmi::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct A;

fn main() -> Result<()> {
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();

    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.
    let module = Module::new(&engine, include_bytes!("../py.wasm"))?;

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    let mut store = Store::new(&engine, "String::new()".to_owned());

    let s = Arc::new("a + 1");

    let export_string = Func::wrap(&mut store, |caller: Caller<'_, _>, addr: i32, len: i32| {
        let m = caller.get_export("memory").unwrap().into_memory().unwrap();
        let slice = &m.data(caller.as_context())[addr as usize..addr as usize + len as usize];
        let str = str::from_utf8(slice).unwrap();
        print!("{str}");
    });

    let import_string_length = Func::wrap(&mut store, |caller: Caller<'_, String>| -> i32 {
        caller.data().len() as i32
    });
    let import_string_data = Func::wrap(&mut store, |mut caller: Caller<'_, String>, addr: i32| {
        let m = caller.get_export("memory").unwrap().into_memory().unwrap();
        let bytes = caller.data().as_bytes().to_owned();
        let mut slice = &mut m.data_mut(caller.as_context_mut())
            [addr as usize..addr as usize + bytes.len() as usize];
        slice.write(&bytes).unwrap();
    });
    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = Linker::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    linker
        .define("host", "export_string", export_string)?
        .define("host", "import_string_length", import_string_length)?
        .define("host", "import_string_data", import_string_data)?;
    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    let hello = instance.get_typed_func::<(), ()>(&store, "a")?;

    // And finally we can call the wasm!
    hello.call(&mut store, ())?;

    Ok(())
}

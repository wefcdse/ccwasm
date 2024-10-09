use std::{
    any::{self, Any},
    borrow::Cow,
    hint::{black_box, unreachable_unchecked},
    mem::forget,
    panic::{self, catch_unwind, PanicInfo},
};

fn main() {
    unsafe { unreachable_unchecked() };
    panic::set_hook(Box::new(|info| {
        dbg!(info.payload().type_id());
    }));

    let a = || {
        panic!("aa{}a", black_box(32));
    };
    dbg!(a.type_id());
    let r: Box<dyn Any + Send> = catch_unwind(a).unwrap_err();
    // drop(r);
    dbg!(r.type_id());
    dbg!(r.is::<&'static str>());
    dbg!(r.downcast_ref::<&'static str>());
    dbg!(r.downcast_ref::<String>());
    // dbg!(r.downcast_ref::<String>());
    // dbg!(any::TypeId::of::<String>());
    // dbg!(any::TypeId::of::<&'static str>());
    // dbg!(any::TypeId::of::<Cow<'static, str>>());
    // dbg!(any::TypeId::of::<()>());
    // dbg!(any::TypeId::of::<str>());
    // dbg!(any::TypeId::of::<PanicInfo>());
    // forget(r);
    println!("here");
}

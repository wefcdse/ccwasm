use crate::lua_api::{failed, success, Exportable, Importable, LuaResult};

pub trait ExportFunc<Args, Out> {
    fn call(&self);
}
#[macro_export]
macro_rules! export_funcs {
    ($(($f:ident, $ename:ident)),*) => {
        $(
            #[no_mangle]
            pub extern "C" fn $ename(){
                $crate::cc_mod::ExportFunc::call(&$f);
            }
        )*

            #[no_mangle]
            pub extern "C" fn export_func() {
                $(
                    stringify!($ename).export();
                )*
            }

    };
}
// #[no_mangle]
// pub extern "C" fn export_func() {
//     stringify!(a).export();
// }

macro_rules! impl_export {
    ($($t:ident),*) => {
        impl<$($t: Importable,)* O: Exportable, F: Fn($($t),*) -> LuaResult<O>>
            ExportFunc<($($t,)*), O> for F
        {
            fn call(&self) {
                let o: LuaResult<O> = (|| {
                    $(
                        #[allow(non_snake_case)]
                        let $t = $t::import()?;
                    )*
                    let out = self($($t),*)?;

                    Ok(out)
                })();
                match o {
                    Ok(o) => {
                        success();
                        o.export();
                    }
                    Err(err) => {
                        failed();
                        err.as_str().export();
                    }
                }
            }
        }
    };
}
impl<O: Exportable, F: Fn() -> LuaResult<O>> ExportFunc<(), O> for F {
    fn call(&self) {
        match self() {
            Ok(o) => {
                success();
                o.export();
            }
            Err(err) => {
                failed();
                err.as_str().export();
            }
        }
    }
}

impl_export!(T0);
impl_export!(T0, T1);
impl_export!(T0, T1, T2);
impl_export!(T0, T1, T2, T3);

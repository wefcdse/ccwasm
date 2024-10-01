pub use lua_result::LuaResult;

mod lua_result {
    pub type LuaResult<T> = Result<T, LuaError>;
    use std::borrow::Cow;
    #[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LuaError(Cow<'static, str>);
    impl<T: ToString> From<T> for LuaError {
        fn from(value: T) -> Self {
            Self(Cow::Owned(value.to_string()))
        }
    }
    impl LuaError {
        pub fn from_str(msg: &'static str) -> Self {
            Self(Cow::Borrowed(msg))
        }
        pub fn from_string(msg: String) -> Self {
            Self(Cow::Owned(msg))
        }
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}
pub trait Exportable {
    fn export(&self);
}
pub trait Importable: Sized {
    fn import() -> LuaResult<Self>;
}
pub(crate) mod debug {
    use super::lua_ffi::{addrof, ffi};
    #[allow(unused)]
    pub fn show_str(s: &str) {
        unsafe {
            ffi::show_str(addrof(s), s.len() as i32);
        }
    }
}

pub use lua_ffi::abort_next_import;
pub use lua_ffi::ffi::Typed;
pub use lua_ffi::next_import_type;
pub use lua_ffi::{failed, success};
pub(crate) mod lua_ffi {
    use ffi::Typed;

    pub(crate) mod ffi {
        use crate::lua_api::{lua_result::LuaError, Importable};

        use super::next_import_type;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Typed {
            None,
            I32,
            I64,
            String,
            F32,
            F64,
            Type,
            Error,
        }
        impl Typed {
            pub(crate) fn from_i32(t: i32) -> Self {
                match t {
                    0 => Typed::None,
                    1 => Typed::I32,
                    2 => Typed::I64,
                    3 => Typed::String,
                    4 => Typed::F32,
                    5 => Typed::F64,
                    6 => Typed::Type,
                    _ => Typed::Error,
                }
            }
        }
        impl Importable for Typed {
            fn import() -> crate::lua_api::LuaResult<Self> {
                if next_import_type() != Typed::Type {
                    Err(LuaError::from_str("not receiving type"))?;
                }
                Ok(Typed::from_i32(unsafe { import_i32() }))
            }
        }
        #[allow(unused)]
        #[link(wasm_import_module = "host")]
        extern "C" {
            pub fn show_str(addr: i32, len: i32);
            pub fn next_type() -> i32;

            pub fn export_string(addr: i32, len: i32);
            pub fn import_string_length() -> i32;
            pub fn import_string_data(addr: i32);

            pub fn import_i32() -> i32;
            pub fn export_i32(data: i32);

            pub fn import_i64() -> i64;
            pub fn export_i64(data: i64);

            pub fn import_f32() -> f32;
            pub fn export_f32(data: f32);

            pub fn import_f64() -> f64;
            pub fn export_f64(data: f64);

            pub fn abort_next_import();
            pub fn success();
            pub fn failed();
        }
    }
    pub fn next_import_type() -> Typed {
        Typed::from_i32(unsafe { ffi::next_type() })
    }
    pub fn addrof<T: ?Sized>(s: *const T) -> i32 {
        s as *const () as usize as i32
    }
    pub fn abort_next_import() {
        unsafe {
            ffi::abort_next_import();
        }
    }
    pub fn success() {
        unsafe {
            ffi::success();
        }
    }
    pub fn failed() {
        unsafe {
            ffi::failed();
        }
    }
}

mod io_impl_string {
    use super::{
        lua_ffi::{
            addrof,
            ffi::{self, Typed},
            next_import_type,
        },
        lua_result::LuaError,
        Exportable, Importable, LuaResult,
    };

    impl Exportable for String {
        fn export(&self) {
            export_string(self);
        }
    }
    impl Exportable for str {
        fn export(&self) {
            export_string(self);
        }
    }
    impl Importable for String {
        fn import() -> LuaResult<Self> {
            import_string()
        }
    }

    fn export_string(s: &str) {
        unsafe {
            ffi::export_string(addrof(s), s.len() as i32);
        }
    }
    fn import_string() -> LuaResult<String> {
        if next_import_type() != Typed::String {
            Err(LuaError::from_str("not receiving String"))?;
        }

        let mut a = vec![0u8; unsafe { ffi::import_string_length() } as usize];
        unsafe {
            let addr = a.as_mut_ptr();
            ffi::import_string_data(addr as *const u8 as usize as i32);
        }
        String::from_utf8(a).map_err(|_| LuaError::from_str("non utf8 string"))
    }
}

mod io_impl_number {
    use super::{
        lua_ffi::{
            ffi::{
                export_f32, export_f64, export_i32, export_i64, import_f32, import_f64, import_i32,
                import_i64, Typed,
            },
            next_import_type,
        },
        lua_result::LuaError,
        Exportable, Importable,
    };

    macro_rules! impl_for {
        ($t:ty, $typname:ident, $if:ident, $of:ident) => {
            impl Importable for $t {
                fn import() -> super::LuaResult<Self> {
                    if next_import_type() != Typed::$typname {
                        return Err(LuaError::from_str(concat!(
                            "not receiving ",
                            stringify!($t)
                        )))?;
                    }
                    Ok(unsafe { $if() })
                }
            }
            impl Exportable for $t {
                fn export(&self) {
                    unsafe {
                        $of(*self);
                    }
                }
            }
        };
    }
    impl_for!(i32, I32, import_i32, export_i32);
    impl_for!(i64, I64, import_i64, export_i64);
    impl_for!(f32, F32, import_f32, export_f32);
    impl_for!(f64, F64, import_f64, export_f64);
}
fn _a() {
    concat!();
    stringify!();
}
mod io_impl_utils {
    use super::{next_import_type, Exportable, Importable, Typed};

    impl Importable for () {
        fn import() -> super::LuaResult<Self> {
            Ok(())
        }
    }
    impl Exportable for () {
        fn export(&self) {}
    }

    impl<T: Importable> Importable for Option<T> {
        fn import() -> super::LuaResult<Self> {
            if next_import_type() == Typed::None {
                Ok(None)
            } else {
                Ok(Some(Importable::import()?))
            }
        }
    }
    impl<T: Exportable> Exportable for Option<T> {
        fn export(&self) {
            match self {
                Some(v) => v.export(),
                None => {}
            }
        }
    }
}

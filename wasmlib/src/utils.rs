use std::fmt::Debug;

use crate::lua_api::LuaResult;

pub trait Debuged<T> {
    fn debuged(self) -> LuaResult<T>;
}
impl<T, E: Debug> Debuged<T> for Result<T, E> {
    fn debuged(self) -> LuaResult<T> {
        Ok(self.map_err(|e| format!("{:?}", e))?)
    }
}

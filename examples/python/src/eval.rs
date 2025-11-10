use std::future::Future;
use std::marker::{PhantomData, PhantomPinned};
use std::task::Poll;

use cc_wasm_api::coroutine::{spawn, yield_now, UnsyncChannel};
use cc_wasm_api::export_funcs;
use cc_wasm_api::lua_api::{Exportable, Importable, LuaResult};
use cc_wasm_api::utils::{Debuged, Number, SyncNonSync};

export_funcs!((aa, a), (aa, eval_result), (aa, eval_string), (bb, b));
static C: SyncNonSync<UnsyncChannel<String>> = SyncNonSync(UnsyncChannel::new(10));
fn aa() {
    spawn(async {
        loop {
            let a: Number = async_eval("return redstone.getAnalogInput(\"left\")")
                .await
                .unwrap();
            let () = async_eval(&format!("redstone.setAnalogOutput(\"right\",{})", a))
                .await
                .unwrap();

            sleep().await;
        }
    });

    // spawn(async {
    //     let a: String = async_eval("return \"32e1e212\"").await.unwrap();
    //     C.insert(a).await;
    // });
}
async fn sleep() {
    let _: LuaResult<()> = async_eval("sleep(0)").await;
}
fn bb() -> Option<String> {
    C.try_get()
}
#[allow(unused)]
#[link(wasm_import_module = "host")]
extern "C" {

    pub fn call_eval(addr: i32, len: i32) -> i32;
    pub fn eval_ready() -> i32;
    pub fn clear_eval();
    pub fn import_from_eval();
}

fn eval(s: &str) -> bool {
    let a = unsafe { call_eval(s as *const str as *const () as usize as i32, s.len() as i32) };
    a != 0
}
fn ready() -> bool {
    let a = unsafe { eval_ready() };
    a != 0
}

struct Eval<'a, O: Importable + Unpin> {
    data: Option<&'a str>,
    out: PhantomData<O>,
}
impl<'a, O: Importable + Unpin> Future for Eval<'a, O> {
    type Output = LuaResult<O>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let unpin = self.get_mut();

        if let Some(v) = &unpin.data {
            if eval(v) {
                unpin.data = None;
            } else {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        }

        if !ready() {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        unsafe {
            import_from_eval();
        }
        let o = O::import();
        unsafe {
            clear_eval();
        }
        Poll::Ready(o)
    }
}

fn async_eval<O: Importable + Unpin + 'static>(s: &str) -> impl '_ + Future<Output = LuaResult<O>> {
    Eval {
        data: Some(s),
        out: PhantomData,
    }
}

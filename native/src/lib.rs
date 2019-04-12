#[macro_use]
extern crate neon;

use neon::prelude::*;

fn adjust(mut cx: FunctionContext) -> JsResult<JsNumber> {
        let work_ema: f64 = cx.argument::<JsNumber>(0)?.value();
        let t_ema: f64 = cx.argument::<JsNumber>(1)?.value();
        let hash_perc: f64 = work_ema / t_ema;
        let val: f64 = 1.0 / (15000.0 * hash_perc);
        let calc: f64 = 1.0 - 0.5_f64.powf(val);
    Ok(cx.number(calc))
}

register_module!(mut cx, {
    cx.export_function("adjust", adjust)
});

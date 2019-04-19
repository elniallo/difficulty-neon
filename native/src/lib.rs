#[macro_use]
extern crate neon;

use neon::prelude::*;

fn adjust(mut cx: FunctionContext) -> JsResult<JsNumber> {
        let work_ema: f64 = cx.argument::<JsNumber>(0)?.value();
        let t_ema: f64 = cx.argument::<JsNumber>(1)?.value();
        let hash_perc: f64 = work_ema / t_ema;
        let calc: f64 = 1.0 - 0.5_f64.powf(t_ema / (15000.0 * work_ema));
    Ok(cx.number(calc))
}
fn calc_ema(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let new_value: f64 = cx.argument::<JsNumber>(0)?.value();
    let previous_value: f64 = cx.argument::<JsNumber>(1)?.value();
    let alpha: f64 = cx.argument::<JsNumber>(2)?.value();
    let res = alpha.mul_add(new_value,((1.0 - alpha) * previous_value));
    Ok(cx.number(res))
    }

register_module!(mut cx, {
    cx.export_function("adjust", adjust);
    cx.export_function("ema",calc_ema)
});

#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js;
mod fibonacci;

fn get_nth_fibonacci(call: Call) -> JsResult<js::JsNumber> {
    let scope = call.scope;
    let num =  call.arguments.require(scope, 0)?.check::<js::JsNumber>()?.value();

    let value = fibonacci::fibonacci(num as i64);
    Ok(js::JsNumber::new(scope, value as f64))
}

register_module!(m, {
    m.export("getNthFibonacci", get_nth_fibonacci)
});

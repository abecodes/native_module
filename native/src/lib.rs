#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
	let greetee = cx.argument::<JsString>(0)?.value();
	Ok(cx.string("Hello ".to_owned() + &greetee))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});

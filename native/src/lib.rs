use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let len = cx.len();
    let mut args: Vec<f64> = Vec::with_capacity(len as usize);
    let mut result = 0 as f64;
    
    for i in 0..len{
        let arg = cx.argument::<JsNumber>(i)?.value();
        args.push(arg);
        result = result + arg;
    }
    println!("参数:{:?}",args);
    let args_str:Vec<String> = args.iter().map(|n| n.to_string()).collect();
    println!("参数:{:?}",args_str.join(","));
    Ok(cx.number(result))
}

fn obj_test(mut cx: FunctionContext) -> JsResult<JsObject> {
    let js_object = JsObject::new(&mut cx);
    let js_str  = cx.string("aa的值");
    js_object.set(&mut cx, "aa", js_str)?;
    let js_str = cx.string("bb的值");
    js_object.set(&mut cx, "bb", js_str)?;
    Ok(js_object)
}

fn cb_test(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let f = cx.argument::<JsFunction>(0)?;
    let args: Vec<Handle<JsValue>> = vec![cx.number(16.0).upcast(),cx.string("hello").upcast()];
    let null = cx.null();
    f.call(&mut cx, null, args)?.downcast::<JsNumber>().or_throw(&mut cx)
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("add",add)?;
    cx.export_function("objTest",obj_test)?;
    cx.export_function("cbTest",cb_test)?;
    Ok({})
});

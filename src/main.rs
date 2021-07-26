
use std::borrow::Borrow;

use deno_core::{JsRuntime, OpState, Resource, error::AnyError};
use gag::BufferRedirect;
use lambda_runtime::{Context, handler_fn};
use serde_json::{Value, json};
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error>{
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(_event: Value, _: Context) -> Result<Value, lambda_runtime::Error> {
    let source = "Deno.core.opSync('op_return', '123');";
    let mut runtime = JsRuntime::new(Default::default());
    runtime.register_op("op_return", deno_core::op_sync(op_return));
    runtime.sync_ops_cache();
    let mut buf = BufferRedirect::stdout().unwrap();
    runtime.execute_script("main.js", &source).unwrap();
    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();
    drop(buf);
    Ok(json!({ "message": format!("{}", output)}))
}

fn op_return(state: &mut OpState, result: String, _: ()) -> Result<(), AnyError> {
    let _rid = state.resource_table.add(Container { value: result.clone() });
    println!("{}", result);
    Ok(())
}

struct Container {
    value: String
}

impl Resource for Container {
   fn close(self: std::rc::Rc<Self>) {}
}
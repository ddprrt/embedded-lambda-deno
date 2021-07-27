use lambda_runtime::{Context, handler_fn};
use serde_json::{Value, json};

use rusty_v8 as v8;

use std::time::SystemTime;


#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error>{
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(_event: Value, _: Context) -> Result<Value, lambda_runtime::Error> {
    let now = SystemTime::now();
    println!("Start: {}", now.elapsed().unwrap().as_millis());
    let platform = v8::new_default_platform(1, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    println!("Platform {}", now.elapsed().unwrap().as_millis());
    
    // Create a new Isolate and make it the current one.
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

    println!("Isolate {}", now.elapsed().unwrap().as_millis());

    // Create a stack-allocated handle scope.
    let handle_scope = &mut v8::HandleScope::new(isolate);

    println!("Scope {}", now.elapsed().unwrap().as_millis());

    // Create a new context.
    let context = v8::Context::new(handle_scope);

    println!("Context {}", now.elapsed().unwrap().as_millis());

    // Enter the context for compiling and running the hello world script.
    let scope = &mut v8::ContextScope::new(handle_scope, context);
    println!("C Scope{}", now.elapsed().unwrap().as_millis());

    // Create a string containing the JavaScript source code.
    let code = v8::String::new(scope, "'Hello' + ' World!'").unwrap();
    println!("Code {}", now.elapsed().unwrap().as_millis());

    // Compile the source code.
    let script = v8::Script::compile(scope, code, None).unwrap();
    println!("Compile {}", now.elapsed().unwrap().as_millis());
    // Run the script to get the result.
    let result = script.run(scope).unwrap();
    println!("Result {}", now.elapsed().unwrap().as_millis());

    // Convert the result to a string and print it.
    let _result = result.to_string(scope).unwrap();


    unsafe {
        v8::V8::dispose();
    }
    println!("Dispose {}", now.elapsed().unwrap().as_millis());
    v8::V8::shutdown_platform();
    println!("Shutdown {}", now.elapsed().unwrap().as_millis());
    Ok(json!({ "message": "yolo" }))
}

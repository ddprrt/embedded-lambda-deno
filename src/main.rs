use lambda_runtime::{Context, handler_fn};
use serde_json::{Value, json};


#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error>{
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(_event: Value, _: Context) -> Result<Value, lambda_runtime::Error> {
    Ok(json!({ "message": "yolo" }))
}

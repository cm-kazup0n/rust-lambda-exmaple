use lambda_runtime::{handler_fn, Error, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Event {
  /// the Answer to the Ultimate Question of Life, the Universe, and Everything 
  answer: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(handler)).await?;
    Ok(())
}

async fn handler(event:Event, context: Context) -> Result<(), Error> {
  println!("{:?}", event);
  println!("{:?}", context);
  Ok(())
}

use neon::prelude::*;
use iota::Client;
use iota_streams::{
    app_channels::api::tangle::{Address, Author, DefaultTW, Message},
    core::tbits::Tbits,
    protobuf3::types::Trytes,
};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello Node.js from Rust."))
}

struct BackgroundTask;

impl Task for BackgroundTask {
    // If the computation does not error, it will return an i32.
    // Otherwise, it will return a String as an error
    type Output = String;
    type Error = String;
    type JsEvent = JsString;

    // Perform expensive computation here. What runs in here
    // will not block the main thread. Will run in a background
    // thread
    fn perform(&self) -> Result<String, String> {

        let seed = "YOURSEED99999999999999999999999999999999999999999999999999999999";
        // tangle client
        let mut api = Client::new("https://nodes.comnet.thetangle.org:443");
        // Create the author
        let mut author = Author::new(seed, 3, true);

        println!(
            "Channel Adress = {}",
            author.channel_address()
        );

        // Demo how to call error
        let result = "pass";
        if result != "pass" {
            return Err("This will fail".to_string());
        }

        Ok(author.channel_address().to_string())
    }

    fn complete(self, mut cx: TaskContext, result: Result<String, String>) -> JsResult<JsString> {
        Ok(cx.string(result.unwrap()))
    }
}

pub fn publish(mut cx: FunctionContext) -> JsResult<JsUndefined> {

    let f = cx.argument::<JsFunction>(0)?;
    BackgroundTask.schedule(f);
   
    Ok(cx.undefined())
}

register_module!(mut m, {
    m.export_function("hello", hello)?;
    m.export_function("publish", publish)?;
    Ok(())
});

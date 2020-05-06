use neon::prelude::*;
use iota::Client;

use iota_streams::{
    app_channels::api::tangle::{Author},
};
use iota_streams::app::transport::tangle::client::SendTrytesOptions;

use crate::api_author::announce::start_a_new_channel;
mod api_author;


fn get_node_info(mut cx: FunctionContext) -> JsResult<JsString> {
    let iota = iota::Client::new("https://nodes.comnet.thetangle.org");
    // https://github.com/wusyong/iota-example/blob/master/src/main.rs
    // let node_info = iota.get_node_info().await?;
    // println!("{:#?}", node_info);
    Ok(cx.string("Node info not implemented yet."))
}

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


        // Create a new channel
        let mut author = Author::new("AUTHORSECRET999", 3, true);

        // Connect to a node and pass this object to the function
        let mut client = iota::Client::new("https://nodes.devnet.iota.org:443");

        // Change the default settings to use a lower minimum weight magnitude for the Devnet
        let mut send_opt = SendTrytesOptions::default();

        // Default MWM is 14
        send_opt.min_weight_magnitude = 9; 

        // TODO: Fix this
        // match start_a_new_channel(&mut author, &mut client, send_opt) {
        //     Ok(()) => (),
        //     Err(error) => println!("Failed with error {}", error),
        // }
    

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
    m.export_function("get_node_info", get_node_info)?;
    Ok(())
});
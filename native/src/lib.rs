use neon::prelude::*;
use iota::Client;


pub mod transport;

use std::{thread, time};

pub mod messaging;
pub mod channels;
use failure::{Fallible};

use channels::channel_author;
use channels::channel_subscriber;


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


        let seed_author = "SOME9AUTHOR9SEED9HERE9NOW9K";
        let seed_subscriber = "SOME9SUBSCRIBER9SEED";
    
        let node = "https://nodes.devnet.iota.org:443";
    
    
    
        //Create Channel Instance for writer
        let mut channel_author = channel_author::Channel::new(seed_author, node);
    
        //Open Channel
        let (channel_address, announcement_tag) = channel_author.open().unwrap();
        println!("channel_address: {}", channel_address);
        println!("announcement_tag: {}", announcement_tag);
    

        // Demo how to call error
        let result = "pass";
        if result != "pass" {
            return Err("This will fail".to_string());
        }

        Ok(channel_address.to_string())
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
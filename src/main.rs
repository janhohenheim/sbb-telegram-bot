extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;
extern crate sbb_telegram_bot;
extern crate reqwest;
extern crate dotenv;
#[macro_use]
extern crate maplit;

use dotenv::dotenv;
use std::env;

use iron::prelude::*;
use iron::status;
use iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain};
use router::NoRoute;
use std::io::Read;
use sbb_telegram_bot::model::telegram;
struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, format!("Invalid request: {}", req.url))))
        } else {
            Err(err)
        }
    }
}

fn main() {
    dotenv().ok();
    let router = router!(telegram: post "/sbb/telegram" => telegram);
    let mut chain = Chain::new(router);
    chain.link_after(Custom404);
    Iron::new(chain).http("localhost:3001").unwrap();


    fn telegram(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body
            .read_to_string(&mut body)
            .map_err(|e| IronError::new(e, (status::BadRequest, "Error reading request")))?;
        let update: telegram::Update = serde_json::from_str(&body).unwrap();
        if let Some(msg) = update.message {
            if let Some(txt) = msg.text {
                if txt == "/start" {
                    let url = format!("{}{}{}",
                                      "https://api.telegram.org/bot",
                                      bot_token(),
                                      "/sendMessage");
                    let params = hashmap![
                        "chat_id" => format!("{}", msg.chat.id),
                        "text" => "If the bot was already working, \
                                    you would now have registered yourself. \
                                    Alas, as this is only a placeholder text, \
                                    nothing happened".to_owned(),
                    ];
                    let client = reqwest::Client::new().map_err(|e| {
                                     IronError::new(e,
                                                    (status::BadRequest, "Error reading request"))
                                 })?;
                    client.post(&url)
                        .json(&params)
                        .send()
                        .unwrap();
                }
            }
        }
        Ok(Response::with((status::Ok, "ok")))
    }
}


fn bot_token() -> String {
    env::var_os("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be specified. \
                Did you forget to add it to your .env file?")
        .into_string()
        .expect("TELEGRAM_BOT_TOKEN does not contain a valid UTF8 string")
}

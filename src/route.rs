extern crate iron;

extern crate serde_json;
extern crate reqwest;

use self::iron::prelude::*;
use self::iron::status;
use self::iron::{ Request, Response, IronResult};

use super::util::bot_token;
use super::model::telegram;

use std::io::Read;


pub fn telegram(req: &mut Request) -> IronResult<Response> {
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
                                                (status::InternalServerError,
                                                 "Error setting up HTTP client"))
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

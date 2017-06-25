extern crate iron;

extern crate serde_json;
extern crate reqwest;

use self::iron::prelude::*;
use self::iron::{status, Request, Response, IronResult};

use super::util::{BotData, read_bot_data, send};
use super::model::telegram;

use std::io::Read;


pub fn telegram(req: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    req.body
        .read_to_string(&mut body)
        .map_err(|e| IronError::new(e, (status::BadRequest, "Error reading request")))?;
    let update: telegram::Update = serde_json::from_str(&body).unwrap();
    if let Some(msg) = update.channel_post {
        if let Some(txt) = msg.text {
            if txt == format!("/start@{}", read_bot_data(&BotData::Name)) {
                respond_start(msg.chat.id)?;
            }
        }
    } else if let Some(msg) = update.message {
        if let Some(txt) = msg.text {
            if txt == "/start" {
                respond_start(msg.chat.id)?;
            } else {
                respond_unknown(msg.chat.id)?;
            }
        }
    }
    Ok(Response::with((status::Ok, "ok")))
}

fn respond_start(chat_id: i32) -> IronResult<reqwest::Response> {
    send(chat_id,
         "If the bot was already working, \
                    you would now have registered yourself. \
                    Alas, as this is only a placeholder text, \
                    nothing happened")
}

fn respond_unknown(chat_id: i32) -> IronResult<reqwest::Response> {
    send(chat_id, "Unknown command. Try using /help")
}

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
    if let Some(msg) = update.message {
        if let Some(txt) = msg.text {
            let is_private = msg.chat.chat_type == "private";
            let identifier = format!("@{}", read_bot_data(&BotData::Name));
            let has_identifier = txt.find(&identifier).is_some();
            let txt = strip_identifier(&txt);
            if is_private || has_identifier {
                let id = msg.chat.id;
                match txt.as_str() {
                    "/start" => respond_start(id),
                    _ => {
                        if is_private {
                            respond_unknown(id)
                        } else {
                            Ok(())
                        }
                    }
                }?;
            }
        }
    }
    Ok(Response::with((status::Ok, "ok")))
}

fn respond_start(chat_id: i32) -> IronResult<()> {
    send(chat_id,
         "If the bot was already working, \
                    you would now have registered yourself. \
                    Alas, as this is only a placeholder text, \
                    nothing happened")?;
    Ok(())
}

fn respond_unknown(chat_id: i32) -> IronResult<()> {
    send(chat_id, "Unknown command. Try using /help")?;
    Ok(())
}

fn strip_identifier(msg: &str) -> String {
    let identifier = format!("@{}", read_bot_data(&BotData::Name));
    if let Some(pos) = msg.find(&identifier) {
        return msg[..pos].to_owned();
    }
    msg.to_owned()
}

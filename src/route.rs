extern crate iron;
extern crate serde_json;
extern crate reqwest;
extern crate csv;

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
                    "/help" => respond_help(id),
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


fn strip_identifier(msg: &str) -> String {
    let identifier = format!("@{}", read_bot_data(&BotData::Name));
    if let Some(pos) = msg.find(&identifier) {
        return msg[..pos].to_owned();
    }
    msg.to_owned()
}

fn respond_start(chat_id: i32) -> IronResult<()> {
    send(chat_id,
         "If the bot was already working, \
            you would now have registered yourself. \
            Alas, as this is only a placeholder text, \
            nothing happened")?;
    Ok(())
}

fn respond_help(chat_id: i32) -> IronResult<()> {
    send(chat_id,
        "Available commands:\n\
        /start: Subscribes this chat to be notified of SBB delays\n\
        /help: Shows this window")?;
    Ok(())
}

fn respond_unknown(chat_id: i32) -> IronResult<()> {
    send(chat_id, "Unknown command. Try using /help")?;
    Ok(())
}

fn broadcast(msg: &str) -> IronResult<()> {
    let id_file = read_bot_data(&BotData::IdFile);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(id_file)
        .expect("error creating csv reading");
    for id in rdr.records() {
        if let Ok(id) = id {
            match id[0].parse::<i32>() {
                Ok(id) => send(id, msg)?,
                Err(e) => {
                    println!("Error converting chat id into i32: {:?}", e);
                    continue;
                }
            };
        } else {
            println!("Error reading chat id: {:?}", id);
        }
    }
    Ok(())
}
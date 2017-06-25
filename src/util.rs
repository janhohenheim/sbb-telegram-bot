extern crate iron;
extern crate serde_json;
extern crate reqwest;

use self::iron::prelude::*;
use self::iron::{status, IronResult};
use std::env;

pub enum BotData {
    Token,
    Name,
    IdFile,
}

pub fn read_bot_data(data: &BotData) -> String {
    let env_var = match *data {
        BotData::Token => "TELEGRAM_BOT_TOKEN",
        BotData::Name => "TELEGRAM_BOT_NAME",
        BotData::IdFile => "TELEGRAM_BOT_CHAT_ID_FILE"
    };
    read_env_var(env_var)
}


fn read_env_var(var: &str) -> String {
    env::var_os(var)
        .expect(&format!("{} must be specified. \
                Did you forget to add it to your .env file?",
                         var))
        .into_string()
        .expect(&format!("{} does not contain a valid UTF8 string", var))
}


pub fn send(chat_id: i32, msg: &str) -> IronResult<reqwest::Response> {
    let url = format!("{}{}{}",
                      "https://api.telegram.org/bot",
                      read_bot_data(&BotData::Token),
                      "/sendMessage");
    let params = hashmap![
                "chat_id" => format!("{}", chat_id),
                "text" => msg.to_owned(),
            ];
    let client = reqwest::Client::new().map_err(|e| {
                     IronError::new(e,
                                    (status::InternalServerError, "Error setting up HTTP client"))
                 })?;
    client.post(&url)
        .json(&params)
        .send()
        .map_err(|e| IronError::new(e, (status::InternalServerError, "Error sending data")))
}

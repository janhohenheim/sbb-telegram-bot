extern crate iron;
extern crate serde_json;
extern crate reqwest;
extern crate csv;

use self::iron::prelude::*;
use self::iron::{status, IronResult};
use std::env;
use std::convert::From;
use std::num::ParseIntError;

pub enum BotData {
    Token,
    Name,
    IdFile,
}

pub fn read_bot_data(data: &BotData) -> String {
    let env_var = match *data {
        BotData::Token => "TELEGRAM_BOT_TOKEN",
        BotData::Name => "TELEGRAM_BOT_NAME",
        BotData::IdFile => "TELEGRAM_BOT_CHAT_ID_FILE",
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

pub fn broadcast(msg: &str) -> Result<(), Vec<BroadcastErr>> {
    let id_file = read_bot_data(&BotData::IdFile);
    let mut errs = Vec::<BroadcastErr>::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(id_file)
        .expect("error creating csv reading");
    for id in rdr.records() {
        match id {
            Ok(id) => {
                match id[0].parse::<i32>() {
                    Ok(id) => {
                        if let Err(e) = send(id, msg) {
                            errs.push(BroadcastErr::from(e))
                        }
                    }
                    Err(e) => errs.push(BroadcastErr::from(e)),
                }
            }
            Err(e) => errs.push(BroadcastErr::from(e)),
        };
    }
    if errs.is_empty() { Ok(()) } else { Err(errs) }
}

pub fn register(chat_id: i32) -> csv::Result<()> {
    let id_file = read_bot_data(&BotData::IdFile);
    let mut wtr = csv::Writer::from_path(id_file)?;
    wtr.write_record(&[format!("{}", chat_id)])?;
    wtr.flush()?;
    Ok(())
}

#[derive(Debug)]
pub enum BroadcastErr {
    Csv(csv::Error),
    Iron(IronError),
    Parse(ParseIntError),
}

impl From<csv::Error> for BroadcastErr {
    fn from(e: csv::Error) -> Self {
        BroadcastErr::Csv(e)
    }
}

impl From<IronError> for BroadcastErr {
    fn from(e: IronError) -> Self {
        BroadcastErr::Iron(e)
    }
}

impl From<ParseIntError> for BroadcastErr {
    fn from(e: ParseIntError) -> Self {
        BroadcastErr::Parse(e)
    }
}

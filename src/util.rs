extern crate iron;
extern crate serde_json;
extern crate reqwest;
extern crate csv;

use self::iron::prelude::*;
use self::iron::{status, IronResult};
use std::env;
use std::convert::From;
use std::num::ParseIntError;
use std::fs::OpenOptions;
use std::error::Error;
use std::fmt;

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

pub fn broadcast(msg: &str) -> Result<(), BroadcastErr> {
    for id in chat_ids()? {
        send(id, msg)?;
    }
    Ok(())
}

pub fn register(chat_id: i32) -> Result<(), BroadcastErr> {
    if chat_ids()?.iter().any(|id| *id == chat_id) {
        return Ok(());
    }
    let id_file = read_bot_data(&BotData::IdFile);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&id_file)
        .expect(&format!("Failed to open file {} in append mode", id_file));
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file);

    wtr.write_record(&[format!("{}", chat_id)])?;
    wtr.flush().expect("Failed to flush CSV writer");
    Ok(())
}

pub fn chat_ids() -> Result<Vec<i32>, BroadcastErr> {
    create_ids_file_if_not_exists();
    let id_file = read_bot_data(&BotData::IdFile);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(id_file)
        .expect("Error creating csv reader. Does the id csv exist?");
    let mut ids = Vec::new();
    for record in rdr.records() {
        ids.push(record?[0].parse::<i32>()?);
    }
    Ok(ids)
}

pub fn create_ids_file_if_not_exists() {
    let id_file = read_bot_data(&BotData::IdFile);
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(&id_file)
        .expect(&format!("Failed to open or create file {}", id_file));
}

#[derive(Debug)]
pub enum BroadcastErr {
    Csv(csv::Error),
    Iron(IronError),
    Parse(ParseIntError),
}

impl fmt::Display for BroadcastErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BroadcastErr::Csv(ref e) => e.fmt(f),
            BroadcastErr::Iron(ref e) => e.fmt(f),
            BroadcastErr::Parse(ref e) => e.fmt(f),
        }
    }
}
impl Error for BroadcastErr {
    fn description(&self) -> &str {
        "Something went wrong while doing csv stuff"
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BroadcastErr::Csv(ref e) => Some(e),
            BroadcastErr::Iron(ref e) => Some(e),
            BroadcastErr::Parse(ref e) => Some(e),
        }
    }
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

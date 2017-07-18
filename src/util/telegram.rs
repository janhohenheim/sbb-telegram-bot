extern crate iron;
extern crate reqwest;
extern crate csv;
extern crate serde_json;

use super::{get_link, read_env_var, EnvVar, create_file_if_not_exists};

use self::iron::prelude::*;
use self::iron::{status, IronResult};
use err::BroadcastErr;
use std::fs::{File, OpenOptions};
use model::telegram::{InlineKeyboardMarkup, InlineKeyboardButton};

pub fn send(chat_id: i64, msg: &str) -> IronResult<reqwest::Response> {
    send_with_markup(chat_id, msg, &None)
}


pub fn send_with_markup(
    chat_id: i64,
    msg: &str,
    markup: &Option<InlineKeyboardMarkup>,
) -> IronResult<reqwest::Response> {
    let url = format!(
        "{}{}{}",
        "https://api.telegram.org/bot",
        read_env_var(&EnvVar::Token),
        "/sendMessage"
    );
    let mut params =
        hashmap![
                "chat_id" => format!("{}", chat_id),
                "text" => msg.to_owned(),
                "parse_mode" => "Markdown".to_owned(),
            ];
    if let Some(ref markup) = *markup {
        params.insert("reply_markup", serde_json::to_string(&markup).unwrap());
    }
    let client = reqwest::Client::new().map_err(|e| {
        IronError::new(e, (
            status::InternalServerError,
            "Error setting up HTTP client",
        ))
    })?;
    client.post(&url).json(&params).send().map_err(|e| {
        IronError::new(e, (status::InternalServerError, "Error sending data"))
    })
}

pub fn broadcast_with_markup(
    msg: &str,
    markup: &Option<InlineKeyboardMarkup>,
) -> Result<(), BroadcastErr> {
    for id in chat_ids()? {
        send_with_markup(id, msg, markup)?;
    }
    Ok(())
}

pub fn broadcast(msg: &str) -> Result<(), BroadcastErr> {
    broadcast_with_markup(msg, &None)
}

pub fn get_info_markup(msg: &str) -> Option<InlineKeyboardMarkup> {
    if let Some(link) = get_link(msg) {
        let button = InlineKeyboardButton::Url {
            text: "info".to_owned(),
            url: link,
        };
        let markup = InlineKeyboardMarkup { inline_keyboard: vec![vec![button]] };
        Some(markup)
    } else {
        None
    }
}

pub fn register(chat_id: i64) -> Result<bool, BroadcastErr> {
    if chat_ids()?.iter().any(|id| *id == chat_id) {
        return Ok(false);
    }
    let id_file = read_env_var(&EnvVar::IdFile);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&id_file)
        .expect(&format!("Failed to open file {} in append mode", id_file));
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(
        file,
    );

    wtr.write_record(&[format!("{}", chat_id)])?;
    wtr.flush().expect("Failed to flush CSV writer");
    Ok(true)
}

pub fn unregister(chat_id: i64) -> Result<bool, BroadcastErr> {
    let mut ids = chat_ids()?;
    let pos = ids.iter().position(|&id| id == chat_id);
    if pos.is_none() {
        return Ok(false);
    }
    let pos = pos.unwrap();
    ids.swap_remove(pos);
    write_chat_ids(&ids);
    Ok(true)
}

fn write_chat_ids(ids: &[i64]) {
    let filename = read_env_var(&EnvVar::IdFile);
    let file = File::create(filename).unwrap();
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(
        file,
    );
    for id in ids {
        wtr.write_record(&[format!("{}", id)]).unwrap();
    }
    wtr.flush().expect("Failed to flush CSV writer");
}

pub fn chat_ids() -> Result<Vec<i64>, BroadcastErr> {
    let id_file = read_env_var(&EnvVar::IdFile);
    create_file_if_not_exists(&id_file);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(id_file)
        .expect("Error creating csv reader. Does the id csv exist?");
    let mut ids = Vec::new();
    for record in rdr.records() {
        ids.push(record?[0].parse::<i64>()?);
    }
    Ok(ids)
}

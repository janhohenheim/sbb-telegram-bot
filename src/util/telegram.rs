extern crate iron;
extern crate reqwest;
extern crate csv;

use super::{read_env_var, EnvVar, create_file_if_not_exists};

use self::iron::prelude::*;
use self::iron::{status, IronResult};
use err::BroadcastErr;
use std::io::Write;
use std::fs::{File, OpenOptions};


pub fn send(chat_id: i32, msg: &str) -> IronResult<reqwest::Response> {
    let url = format!("{}{}{}",
                      "https://api.telegram.org/bot",
                      read_env_var(&EnvVar::Token),
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

pub fn register(chat_id: i32) -> Result<bool, BroadcastErr> {
    if chat_ids()?.iter().any(|id| *id == chat_id) {
        return Ok(false);
    }
    let id_file = read_env_var(&EnvVar::IdFile);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&id_file)
        .expect(&format!("Failed to open file {} in append mode", id_file));
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file);

    wtr.write_record(&[format!("{}", chat_id)])?;
    wtr.flush().expect("Failed to flush CSV writer");
    Ok(true)
}

pub fn unregister(chat_id: i32) -> Result<bool, BroadcastErr> {
    let mut ids = chat_ids()?;
    let pos = ids.iter().position(|&id| id == chat_id);
    if let None = pos {
        return Ok(false);
    }
    let pos = pos.unwrap();
    ids.swap_remove(pos);
    write_chat_ids(&ids);
    Ok(true)
}

fn write_chat_ids(ids: &Vec<i32>) {
    let filename = read_env_var(&EnvVar::IdFile);
    let mut file = File::create(filename).unwrap();
    let mut content = String::new();
    for id in ids {
        content += &format!("{}", id);
    }
    file.write_all(&content.into_bytes()).unwrap();
}

pub fn chat_ids() -> Result<Vec<i32>, BroadcastErr> {
    let id_file = read_env_var(&EnvVar::IdFile);
    create_file_if_not_exists(&id_file);
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

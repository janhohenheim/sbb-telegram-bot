extern crate iron;
extern crate serde_json;
extern crate reqwest;
use self::iron::prelude::*;
use self::iron::{status, Request, Response, IronResult};

use util::{EnvVar, read_env_var};
use util::telegram::{register, unregister, send, send_with_reply_markup};
use util::twitter;
use model::telegram;

use std::io::Read;


pub fn telegram(req: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    req.body
        .read_to_string(&mut body)
        .map_err(|e| IronError::new(e, (status::BadRequest, "Error reading request")))?;
    let update: telegram::Update = serde_json::from_str(&body).unwrap();
    println!("{:?}", update);
    if let Some(msg) = update.message {
        if let Some(txt) = msg.text {
            let is_private = msg.chat.chat_type == "private";
            let identifier = format!("@{}", read_env_var(&EnvVar::Name));
            let has_identifier = txt.find(&identifier).is_some();
            let txt = strip_identifier(&txt);
            if is_private || has_identifier {
                let id = msg.chat.id;
                match txt.as_str() {
                    "/start" => respond_start(id),
                    "/help" => respond_help(id),
                    "/unsubscribe" => respond_unsubscribe(id),
                    "/delays" => respond_delays(id),
                    txt => {
                        if let Ok(num) = txt.parse::<u32>() {
                            respond_num_delays(id, num)
                        } else if is_private {
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
    let identifier = format!("@{}", read_env_var(&EnvVar::Name));
    let msg = if let Some(pos) = msg.find(&identifier) {
        let end = pos + identifier.len();
        msg[..pos].to_owned() + &msg[end..]
    } else {
        msg.to_owned()
    };
    msg.trim().to_owned()
}

fn respond_start(chat_id: i64) -> IronResult<()> {
    let new_registration = register(chat_id).unwrap();
    let msg = if new_registration {
        let acc = read_env_var(&EnvVar::TwitterAcc);
        let tweet = twitter::user_last_tweet(&acc).unwrap();
        format!("Successfully registered!\nLast delay:\n\n{}", tweet)
    } else {
        "This chat has already been registered".to_owned()
    };
    send(chat_id, &msg)?;
    Ok(())
}

fn respond_help(chat_id: i64) -> IronResult<()> {
    send(chat_id,
         "Available commands:\n\n\
        /start - Subscribes this chat to be notified of SBB delays\n\
        /unsubscribe - Unsubscribes this chat from the delay notifications\n\
        /delays - Shows the last `n` delays\n\
        /help - Shows this window")?;
    Ok(())
}

fn respond_unsubscribe(chat_id: i64) -> IronResult<()> {
    let unsubscribed = unregister(chat_id).unwrap();
    let msg = if unsubscribed {
        "Successfully unsubscribed from the delay notifications.\n\
        Use /start to subscribe again"
    } else {
        "This chat was not subscribed to be notified of delays in the first place, silly!\n\
        Use /start if you want to subscribe"
    };
    send(chat_id, msg)?;
    Ok(())
}

fn respond_delays(chat_id: i64) -> IronResult<()> {
    use model::telegram::{InlineKeyboardMarkup, InlineKeyboardButton};
    let mut buttons = Vec::new();
    let mut count = 1;
    for _ in 0..3 {
        let mut row = Vec::new();
        for _ in 0..3 {
            let count_str = format!("{}", count);
            let button = InlineKeyboardButton {
                text: count_str.clone(),
                switch_inline_query_current_chat: Some(count_str),
            };
            row.push(button);
            count += 1;
        }
        buttons.push(row);
    }
    let markup = InlineKeyboardMarkup { inline_keyboard: buttons };
    let msg = "How many of the last delays to you want to look up?";
    let mut s = String::new();
    send_with_reply_markup(chat_id, msg, Some(markup))?.read_to_string(&mut s).unwrap();
    Ok(())
}

fn respond_num_delays(chat_id: i64, delay_count: u32) -> IronResult<()> {
    const MIN: u32 = 1;
    const MAX: u32 = 9;
    if delay_count < MIN || delay_count > MAX {
        let msg = format!("Invalid number of delays selected, \
            please enter a number between {} and {}.",
                          MIN,
                          MAX);
        send(chat_id, &msg)?;
        return Ok(());
    }
    let acc = read_env_var(&EnvVar::TwitterAcc);
    let tweets = twitter::user_timeline(&acc, delay_count).unwrap();
    let mut msg = if delay_count == 1 {
        "Showing last delay:\n\n".to_owned()
    } else {
        format!("Showing last {} delays:\n\n", delay_count)
    };
    for (i, tweet) in tweets.iter().enumerate() {
        msg += &format!("{}. {}\n", i + 1, tweet);
    }
    send(chat_id, &msg)?;
    Ok(())
}

fn respond_unknown(chat_id: i64) -> IronResult<()> {
    send(chat_id, "Unknown command. Try using /help")?;
    Ok(())
}

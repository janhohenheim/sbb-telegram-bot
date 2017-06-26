extern crate iron;
#[macro_use]
extern crate router;
extern crate dotenv;
extern crate sbb_telegram_bot;

use dotenv::dotenv;
use iron::prelude::*;
use sbb_telegram_bot::route;
use sbb_telegram_bot::util::{telegram, twitter};
use std::thread;
use std::time::Duration;

fn main() {
    dotenv().ok();
    thread::spawn(broadcast_loop);
    let router = router!(telegram: post "/sbb/telegram" => route::telegram);
    Iron::new(router).http("localhost:3001").unwrap();
}

fn broadcast_loop() {
    loop {
        thread::sleep(Duration::from_secs(10));
        let tweets = twitter::user_timeline("railinfo_sbb", 1).unwrap();
        let tweet = &tweets[0];
        if tweet.id != twitter::read_last_tweet_id() {
            twitter::write_last_tweet_id(tweet.id);
            let mut txt = tweet.text.clone();
            if let Some(pos) = txt.find("http") {
                txt = txt[..pos].to_owned();
            }
            telegram::broadcast(&txt).unwrap();
        }
    }
}

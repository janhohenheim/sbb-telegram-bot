extern crate iron;
#[macro_use]
extern crate router;
extern crate dotenv;
extern crate sbb_telegram_bot;

use dotenv::dotenv;
use iron::prelude::*;
use sbb_telegram_bot::{route, util};
use sbb_telegram_bot::api::twitter::user_timeline;
use std::thread;
use std::time::Duration;

fn main() {
    dotenv().ok();
    thread::spawn(broadcast_loop);
    let router = router!(telegram: post "/sbb/telegram" => route::telegram);
    Iron::new(router).http("localhost:3001").unwrap();
}

fn broadcast_loop() {
    let mut last_id = 0;
    loop {
        thread::sleep(Duration::from_secs(5));
        let tweets = user_timeline("railinfo_sbb", 1).unwrap();
        let tweet = &tweets[0];
        if tweet.id != last_id {
            last_id = tweet.id;
            util::broadcast(&tweet.text).unwrap();
        }
    }
}

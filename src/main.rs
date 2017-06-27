extern crate iron;
#[macro_use]
extern crate router;
extern crate dotenv;
extern crate sbb_telegram_bot;

use dotenv::dotenv;
use iron::prelude::*;
use sbb_telegram_bot::route;
use sbb_telegram_bot::util::{read_env_var, EnvVar, telegram, twitter};
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
        let acc = read_env_var(&EnvVar::TwitterAcc);
        let tweet = twitter::user_last_tweet(&acc).unwrap();
        if tweet.id != twitter::read_last_tweet_id() {
            twitter::write_last_tweet_id(tweet.id);
            let markup = telegram::get_info_markup(&tweet.text);
            let txt = format!("{}", tweet);
            telegram::broadcast_with_markup(&txt, &markup).unwrap();
        }
    }
}

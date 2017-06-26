extern crate iron;
#[macro_use]
extern crate router;
extern crate dotenv;
extern crate sbb_telegram_bot;

use dotenv::dotenv;
use iron::prelude::*;
use sbb_telegram_bot::{route, util};
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
        thread::sleep(Duration::from_secs(60*60));
        let msg = "An SBB delay occured!\n\
            Haha! Got you there, this is just a dummy text 😜";
        util::broadcast(msg).unwrap();
    }
}

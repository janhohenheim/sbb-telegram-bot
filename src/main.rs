extern crate iron;
#[macro_use]
extern crate router;
extern crate dotenv;
extern crate sbb_telegram_bot;

use dotenv::dotenv;
use iron::prelude::*;
use sbb_telegram_bot::{route, util};

fn main() {
    dotenv().ok();
    let router = router!(telegram: post "/sbb/telegram" => route::telegram);
    Iron::new(router).http("localhost:3001").unwrap();
    util::broadcast("An SBB delay occured!\n\
                Haha got you, this is just a dummy text 😜")
            .unwrap();

}

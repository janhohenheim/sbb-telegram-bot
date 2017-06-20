extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;
extern crate sbb_telegram_bot;


use iron::prelude::*;
use iron::status;
use iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain};
use router::NoRoute;
use std::io::Read;
use sbb_telegram_bot::model::telegram;

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, format!("Invalid request: {}", req.url))))
        } else {
            Err(err)
        }
    }
}

fn main() {
    let router = router!(telegram: post "/sbb/telegram" => telegram);
    let mut chain = Chain::new(router);
    chain.link_after(Custom404);
    Iron::new(chain).http("localhost:3001").unwrap();

    fn telegram(req: &mut Request) -> IronResult<Response> {
        let mut body = Vec::new();
        req
            .body
            .read_to_end(&mut body)
            .map_err(|e|
                IronError::new(e,
                    (status::InternalServerError, "Error reading request")
                )
            )?;
        let body = String::from_utf8(body).unwrap();
        let update: telegram::Update = serde_json::from_str(&body).unwrap();
        if let Some(msg) = update.message {
            if let Some(txt) = msg.text {
                println!("{}", txt);
            }
        }
        Ok(Response::with((status::Ok, "ok")))
    }
}

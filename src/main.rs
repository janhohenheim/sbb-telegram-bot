#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate serde;
extern crate rocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;  

use rocket::response::content::JSON;

#[post("/sbb/telegram/update", format = "application/json", data = "<update>")]
fn telegram_update(update: JSON<Update>) -> String {
    "hi".to_owned()
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

#[derive(Serialize, Deserialize)]
struct Update {
    update_id: i32,
    message: Option<Message>,
    edited_message: Option<Message>,
    channel_post: Option<Message>,
    edited_channel_post: Option<Message>,
    inline_query: Option<InlineQuery>,
    chosen_inline_result: Option<ChosenInlineResult>,
    callback_query: Option<CallbackQuery>,
    shipping_query: Option<ShippingQuery>,
    pre_checkout_query: Option<PreCheckoutQuery>,
}

#[derive(Serialize, Deserialize)]
struct Message {

}

#[derive(Serialize, Deserialize)]
struct InlineQuery {

}

#[derive(Serialize, Deserialize)]
struct ChosenInlineResult {

}

#[derive(Serialize, Deserialize)]
struct CallbackQuery {

}

#[derive(Serialize, Deserialize)]
struct ShippingQuery {

}

#[derive(Serialize, Deserialize)]
struct PreCheckoutQuery {

}

#[derive(Serialize, Deserialize)]
struct User {

}

#[derive(Serialize, Deserialize)]
struct Chat {
    
}
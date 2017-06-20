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
    update_id: i32,  
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}

#[derive(Serialize, Deserialize)]
struct ChosenInlineResult {
    update_id: i32,  
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}

#[derive(Serialize, Deserialize)]
struct CallbackQuery {
    result: i32,  
    from: User,
    message: Option<String>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>
}

#[derive(Serialize, Deserialize)]
struct ShippingQuery {
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<ShippingOption>>,
    error_message: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ShippingOption {
    id:String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Serialize, Deserialize)]
struct ShippingAddress {
   country_code: String,
   state: String,
   city: String,
   street_line1: String,
   street_line2: String,
   post_code: String,
}

#[derive(Serialize, Deserialize)]
struct PreCheckoutQuery {
id:String,
from: User,
currency: String,
total_amount: i32,
invoice_payload: String,
shipping_option_id: Option<String>,
order_info: Option<OrderInfo>,
}

#[derive(Serialize, Deserialize)]
struct User {

}

#[derive(Serialize, Deserialize)]
struct Chat {
    
}

#[derive(Serialize, Deserialize)]
struct Location {
longitude: f64,
latitude: f64,
}

#[derive(Serialize, Deserialize)]
struct OrderInfo {
name: Option<String>,
phone_number: Option<String>,
email: Option<String>,
shipping_address: Option<String>,
}
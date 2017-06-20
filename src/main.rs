extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use iron::status;
use router::Router;

use std::io::Read;
fn main() {   
    let router = router!(telegram: post "/telegram" => telegram);

    Iron::new(router).http("localhost:3000").unwrap();

    fn telegram(req: &mut Request) -> IronResult<Response> {
        let mut body = Vec::new();
        req
            .body
            .read_to_end(&mut body)
            .map_err(|e| IronError::new(e, (status::InternalServerError, "Error reading request")))?;
        let body = String::from_utf8(body).unwrap();
        println!("{}", body);
        Ok(Response::with((status::Ok, "ok")))
    }
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
    message_id: i32,
    from: Option<User>,
    date: i32,
    chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<i32>,
    forward_date: Option<i32>,
    reply_to_message: Box<Option<Message>>,
    edit_date: Option<i32>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    audio: Option<Audio>,
    document: Option<Document>,
    game: Option<Game>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    video: Option<Video>,
    voice: Option<Voice>,
    video_note: Option<VideoNote>,
    new_chat_members: Option<Vec<User>>,
    caption: Option<String>,
    contact: Option<Contact>,
    location: Option<Location>,
    venue: Option<Venue>,
    new_chat_member: Option<User>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct InlineQuery {}

#[derive(Serialize, Deserialize)]
struct ChosenInlineResult {}

#[derive(Serialize, Deserialize)]
struct CallbackQuery {}

#[derive(Serialize, Deserialize)]
struct ShippingQuery {}

#[derive(Serialize, Deserialize)]
struct PreCheckoutQuery {}

#[derive(Serialize, Deserialize)]
struct User {}

#[derive(Serialize, Deserialize)]
struct Chat {}

#[derive(Serialize, Deserialize)]
struct MessageEntity {}


#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
    pub shipping_query: Option<ShippingQuery>,
    pub pre_checkout_query: Option<PreCheckoutQuery>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub date: i32,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i32>,
    pub forward_date: Option<i32>,
    pub reply_to_message: Box<Option<Message>>,
    pub edit_date: Option<i32>,
    pub text: Option<String>,
    pub new_chat_member: Option<User>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub delete_chat_photo: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    pub update_id: i64,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChosenInlineResult {
    pub update_id: i64,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct CallbackQuery {
    pub result: i32,
    pub from: User,
    pub message: Option<String>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct ShippingQuery {
    pub shipping_query_id: String,
    pub ok: bool,
    pub shipping_options: Option<Vec<ShippingOption>>,
    pub error_message: Option<String>,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct ShippingOption {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}


#[derive(Serialize,Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<String>,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}


#[derive(Serialize,Deserialize, Debug)]
#[serde(untagged)]
pub enum InlineKeyboardButton {
    Url { text: String, url: String },
    SwitchInlineQueryCurrentChat {
        text: String,
        switch_inline_query_current_chat: String,
    },
}

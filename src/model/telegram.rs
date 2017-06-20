
#[derive(Serialize, Deserialize)]
pub struct Update {
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
pub struct Message {
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
    new_chat_member: Option<User>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    delete_chat_photo: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct InlineQuery {
    update_id: i32,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChosenInlineResult {
    update_id: i32,
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}

#[derive(Serialize, Deserialize)]
pub struct CallbackQuery {
    result: i32,
    from: User,
    message: Option<String>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingQuery {
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<ShippingOption>>,
    error_message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingOption {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: i32,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}


#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    first_name: Option<String>,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    id: i32,
    #[serde(rename = "type")]
    chat_type: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    all_members_are_administrators: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    longitude: f64,
    latitude: f64,
}

#[derive(Serialize, Deserialize)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<String>,
}

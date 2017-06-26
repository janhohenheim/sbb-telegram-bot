extern crate chrono;
use self::chrono::prelude::*;


#[derive(Deserialize)]
pub struct Tweet {
    pub id: i64,
    pub created_at: DateTime<Local>,
    pub text: String,
}

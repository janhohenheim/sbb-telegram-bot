extern crate reqwest;
extern crate serde_json;

use self::reqwest::Url;

use model::twitter::Tweet;
use util::{BotData, read_bot_data};
use std::io::Read;

pub fn user_timeline(screen_name: &str, count: i32) -> Result<Vec<Tweet>, reqwest::Error> {
    let url = Url::parse_with_params("https://api.twitter.com/1.1/statuses/user_timeline.json",
                                     &[("screen_name", screen_name),
                                       ("count", &format!("{}", count))])
            .unwrap();
    let auth = format!("Bearer {}", read_bot_data(&BotData::BearerToken));
    let mut auth_header = reqwest::header::Headers::new();
    auth_header.set_raw("Authorization", vec![auth.into_bytes()]);

    let mut res = String::new();
    let client = reqwest::Client::new()?;
    client.get(url)
        .headers(auth_header)
        .send()?
        .read_to_string(&mut res)
        .unwrap();

    let tweets: Vec<Tweet> = serde_json::from_str(&res).unwrap();
    Ok(tweets)
}

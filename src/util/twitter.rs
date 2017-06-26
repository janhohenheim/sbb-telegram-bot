extern crate reqwest;
extern crate serde_json;

use super::{read_env_var, EnvVar, create_file_if_not_exists};
use self::reqwest::Url;

use model::twitter::Tweet;
use std::io::{Read, Write};
use std::fs::File;

pub fn user_timeline(screen_name: &str, count: i32) -> Result<Vec<Tweet>, reqwest::Error> {
    let url = Url::parse_with_params("https://api.twitter.com/1.1/statuses/user_timeline.json",
                                     &[("screen_name", screen_name),
                                       ("count", &format!("{}", count))])
            .unwrap();
    let auth = format!("Bearer {}", read_env_var(&EnvVar::BearerToken));
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


pub fn read_last_tweet_id() -> i64 {
    let filename = read_env_var(&EnvVar::LastTweetFile);
    create_file_if_not_exists(&filename);
    let mut file = File::open(filename).unwrap();
    let mut id = String::new();
    file.read_to_string(&mut id).unwrap();
    id.parse::<i64>().unwrap_or_else(|_| {write_last_tweet_id(0); 0}) 
    
}

pub fn write_last_tweet_id(id: i64) {
    let filename = read_env_var(&EnvVar::LastTweetFile);
    let mut file = File::create(filename).unwrap();
    let content = format!("{}", id).into_bytes();
    file.write_all(&content).unwrap();
}

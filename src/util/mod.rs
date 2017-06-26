extern crate iron;
extern crate serde_json;
extern crate reqwest;
extern crate csv;

use std::env;
use std::fs::OpenOptions;

pub mod telegram;
pub mod twitter;

pub enum EnvVar {
    Token,
    Name,
    IdFile,
    BearerToken,
    LastTweetFile,
}

pub fn read_env_var(data: &EnvVar) -> String {
    let env_var = match *data {
        EnvVar::Token => "TELEGRAM_BOT_TOKEN",
        EnvVar::Name => "TELEGRAM_BOT_NAME",
        EnvVar::IdFile => "TELEGRAM_BOT_CHAT_ID_FILE",
        EnvVar::BearerToken => "TWITTER_BEARER_TOKEN",
        EnvVar::LastTweetFile => "LAST_TWEET_FILE",
    };
    read_raw_env_var(env_var)
}


fn read_raw_env_var(var: &str) -> String {
    env::var_os(var)
        .expect(&format!("{} must be specified. \
                Did you forget to add it to your .env file?",
                         var))
        .into_string()
        .expect(&format!("{} does not contain a valid UTF8 string", var))
}




fn create_file_if_not_exists(name: &str) {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(&name)
        .expect(&format!("Failed to open or create file {}", name));
}

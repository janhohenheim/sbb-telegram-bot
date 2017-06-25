use std::env;

pub fn bot_token() -> String {
    env::var_os("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be specified. \
                Did you forget to add it to your .env file?")
        .into_string()
        .expect("TELEGRAM_BOT_TOKEN does not contain a valid UTF8 string")
}

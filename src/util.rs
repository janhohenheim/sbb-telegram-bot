use std::env;

pub enum BotData {
    Token,
    Name,
}

pub fn read_bot_data(data: BotData) -> String {
    let env_var = match data {
        BotData::Token => "TELEGRAM_BOT_TOKEN",
        BotData::Name => "TELEGRAM_BOT_NAME",
    };
    read_env_var(&env_var)
}


fn read_env_var(var: &str) -> String {
    env::var_os(var)
        .expect(&format!("{} must be specified. \
                Did you forget to add it to your .env file?", var))
        .into_string()
        .expect(&format!("{} does not contain a valid UTF8 string", var))
}
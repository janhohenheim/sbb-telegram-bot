#[derive(Deserialize, Debug)]
pub struct Tweet {
    pub id: i64,
    pub text: String,
}

use std::fmt::{self, Display, Formatter};
impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut txt = self.text.clone();
        if let Some(pos) = txt.find("#") {
            txt = txt[..pos].to_owned();
        }
        write!(f, "{}", txt)
    }
}

use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct Tweet {
    pub id: i64,
    pub text: String,
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut txt = self.text.clone();
        if let Some(pos) = txt.find("Einschränkung") {
            txt.insert_str(pos, "⚠️ ");
        }
        if let Some(pos) = txt.find("Unterbruch") {
            txt.insert_str(pos, "🚫 ");
        }
        if let Some(pos) = txt.find(':') {
            txt.insert(0, '*');
            txt.insert(pos + 1, '*');
        }

        if let Some(pos) = txt.find("http") {
            txt = txt[..pos].to_owned();
        }
        write!(f, "{}", txt)
    }
}

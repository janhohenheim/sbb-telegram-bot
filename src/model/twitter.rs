#[derive(Deserialize, Debug)]
pub struct Tweet {
    pub id: i64,
    pub text: String,
}

use std::fmt::{self, Display, Formatter};
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
            txt.insert_str(pos, "[info](");
            let end = {
                let after_link = &txt[pos..];
                let link_end =
                    after_link.find(char::is_whitespace).unwrap_or_else(|| after_link.len());
                pos + link_end
            };
            txt.insert_str(end, ")");
        }
        if let Some(pos) = txt.find('#') {
            txt = txt[..pos].to_owned();
        }
        write!(f, "{}", txt)
    }
}

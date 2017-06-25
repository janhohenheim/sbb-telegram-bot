#[derive(Serialize, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub text: String,
}
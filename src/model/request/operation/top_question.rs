use serde::Deserialize;
#[derive(Deserialize)]
pub struct TopQuestion {
    #[serde(rename = "questionCode")]
    pub question_code: String,
}
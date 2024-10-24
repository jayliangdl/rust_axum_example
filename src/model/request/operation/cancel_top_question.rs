use serde::Deserialize;
#[derive(Deserialize)]
pub struct CancelTopQuestion {
    #[serde(rename = "questionCode")]
    pub question_code: String,
}
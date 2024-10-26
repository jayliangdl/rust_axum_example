use serde::{Deserialize, Serialize};
#[derive(Deserialize,Debug,Serialize)]
pub struct GetQuestionByCode{
    #[serde(rename="questionCode")]
    pub question_code:String
}

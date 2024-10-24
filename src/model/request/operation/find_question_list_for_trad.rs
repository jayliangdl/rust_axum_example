
use serde::{Serialize,Deserialize};
use validator::Validate;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct FindQuestionListForTrad {

    #[serde(rename = "pageNum")]
    #[validate(range(min = 1, message = "页面参数不合法，应该是大于等于1的整数"))]
    pub current_pageno:i64,
    #[serde(rename = "pageSize")]
    pub page_size:i64,

    #[serde(rename="idStr")]
    pub id_str: Option<String>,
    #[serde(rename = "productCode")]
    pub product_code: Option<String>,
    #[serde(rename = "questionContent")]
    pub question_content: Option<String>,
    #[serde(rename = "questionCode")]
    pub question_code: Option<String>,
    #[serde(rename = "creatorName")]
    pub create_name: Option<String>,
    #[serde(rename = "answerCode")]
    pub answer_code: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::NaiveDateTime>,
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::NaiveDateTime>,
    #[serde(rename = "answerContent")]
    pub answer_content:Option<String>
}

impl FindQuestionListForTrad {
    pub fn new() -> Self {
        FindQuestionListForTrad {
            current_pageno: 1,
            page_size: 10,
            id_str: None,
            product_code: None,
            question_content: None,
            question_code: None,
            create_name: None,
            answer_code: None,
            start_time: None,
            end_time: None,
            answer_content: None,
        }
    }
}

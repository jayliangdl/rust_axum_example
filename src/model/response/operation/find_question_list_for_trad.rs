
use serde::{Serialize,Deserialize};
use validator::Validate;
// use std::collections::HashMap;
use crate::model::db::qa::Question as DbQuestion;
use crate::model::db::qa::Answer as DbAnswer;
// use crate::model::db::qa::Page as DbPage;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct FindQuestionListForTrad {
    pub page:Page,
    #[serde(rename = "totalCount")]
    pub total_records:i64,
    pub list:Vec<Question>,
}

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct Page{
    #[serde(rename = "totalCount")]
    pub total_records:i64,
    #[serde(rename = "curPageNum")]
    pub current_pageno:i64,
    #[serde(rename = "pageSize")]
    pub page_size:i64,
    #[serde(rename = "totalPage")]
    pub total_pages:i64,
}

impl FindQuestionListForTrad {
    pub fn new(total_records:i64,current_pageno:i64,page_size:i64,total_pages:i64,list:Vec<Question>) -> Self {
        let page = Page{
            total_records,
            current_pageno,
            page_size,
            total_pages
        };
        FindQuestionListForTrad {
            total_records,
            page,
            list,
        }
    }

//     pub fn from_db_questions(page_with_question : DbPage<DbQuestion>)->Self{
//         let list = vec![];
//         let total_records = page_with_question.total_records;
//         let total_pages = page_with_question.total_pages;
//         let current_pageno = page_with_question.current_pageno;
//         let page_size = page_with_question.page_size;

//         let db_questions = page_with_question.data;
// -
//         db_questions.into_iter().map(|db_question|{
//             let question_code = db_question.question_code.clone();
//             db_question.answers = vec![];
//             let answers = vec![];
//             Question::from_db_questions(Some(&question),answers).unwrap()
//         }).collect();

//         let list = questions.into_iter().map(|question|{
//             let db_answers = question.answers;
//             Question::from_db_questions(Some(&question),Some(answers)).unwrap()
//         }).collect();

//     }
}
#[derive(Serialize,Deserialize, Debug, Validate, Clone  )]
pub struct Question {
    #[serde(rename="idStr")]
    pub id_str: String,
    pub id: i64,
    #[serde(rename = "questionCode")]
    pub question_code: String,
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[serde(rename = "questionContent")]
    pub question_content: String,
    #[serde(rename = "creatorName")]
    pub creator_name: Option<String>,
    #[serde(rename = "createUserId")]
    pub create_user_id: Option<String>,    
    pub sort: i32,
    pub rank: i32,
    #[serde(rename = "createTime")]
    pub create_time: String,
    pub answers: Vec<Answer>,
}

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct Answer {
    #[serde(rename="id")]
    pub id: i64,
    #[serde(rename = "questionCode")]
    pub question_code: String,
    #[serde(rename = "answerContent")]
    pub answer_content: String,
    #[serde(rename = "createUserId")]
    pub create_user_id: Option<String>,
    #[serde(rename = "creatorName")]
    pub creator_name: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    pub status: i8,
}

impl Answer{
    pub fn from_db_answer(answer:DbAnswer)->Self{
        Self{
            id: answer.id.unwrap_or(0),
            question_code: answer.question_code.clone(),
            answer_content: answer.answer_content.clone(),
            create_user_id: answer.create_user_id.clone(),
            creator_name: answer.creator_name.clone(),
            create_time: answer.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: answer.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            status: answer.status,
        }
    }
}

impl Question {
    pub fn from_db_questions(question:DbQuestion,answers:Vec<DbAnswer>)->Self{
        Self{                
            id_str: question.id.unwrap_or(0).to_string(),
            id: question.id.unwrap_or(0),
            product_code: question.product_code.clone(),
            question_content: question.question_content.clone(),
            question_code: question.question_code.clone(),
            creator_name: question.creator_name.clone(),
            create_user_id: question.create_user_id.clone(),
            sort: question.sort,
            rank: question.rank,
            create_time: question.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            answers: answers.into_iter().map(|answer|{
                Answer::from_db_answer(answer)
            }).collect(),
        }
    }
}

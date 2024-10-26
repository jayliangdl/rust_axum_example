use serde::Serialize;
use crate::model::db::qa::{Question as DbQuestion, Answer as DbAnswer };
use crate::model::cache::qa::{Question as CacheQuestion, Answer as CacheAnswer };
#[derive(Serialize,Debug)]
pub struct GetQuestionByCode{
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
    pub update_time: String,
    pub answers: Vec<AnswerGetQuestionByCode>,
}

#[derive(Serialize,Debug)]
pub struct AnswerGetQuestionByCode {
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


impl GetQuestionByCode{
    pub fn from_cache(question_option:Option<CacheQuestion>)->Option<Self>{
        if let Some(question)=question_option{
            let response = Self{
                id_str: question.id.to_string(),
                id: question.id,
                question_code: question.question_code,
                product_code: question.product_code,
                question_content: question.question_content,
                creator_name: question.creator_name,
                create_user_id: question.create_user_id,
                sort: question.sort,
                rank: question.rank,
                create_time: question.create_time.to_string(),
                update_time: question.update_time.to_string(),
                answers:question.answer_list.into_iter().map(|answer|AnswerGetQuestionByCode::from_cache(answer)).collect(),
            };
            Some(response)
        }else{
            None
        }
    }
    pub fn from_db(db_question:DbQuestion,db_answer:Vec<DbAnswer>)->Self{
        let answers = db_answer.into_iter().map(|answer|AnswerGetQuestionByCode::from_db(answer)).collect();
        let response = Self{
            id_str: db_question.id.unwrap_or(0).to_string(),
            id: db_question.id.unwrap_or(0),
            question_code: db_question.question_code,
            product_code: db_question.product_code,
            question_content: db_question.question_content,
            creator_name: db_question.creator_name,
            create_user_id: db_question.create_user_id,
            sort: db_question.sort,
            rank: db_question.rank,
            create_time: db_question.create_time.to_string(),
            update_time: db_question.update_time.to_string(),
            answers,
        };
        response
    }
}

impl AnswerGetQuestionByCode{
    pub fn from_cache(answer:CacheAnswer)->Self{
        Self{
            id: answer.id,
            question_code: answer.question_code,
            answer_content: answer.answer_content,
            create_user_id: answer.create_user_id,
            creator_name: answer.creator_name,
            create_time: answer.create_time.to_string(),
            update_time: answer.update_time.to_string(),
            status: answer.status,
        }
    }
    pub fn from_db(answer:DbAnswer)->Self{
        Self{
            id: answer.id.unwrap_or(0),
            question_code: answer.question_code,
            answer_content: answer.answer_content,
            create_user_id: answer.create_user_id,
            creator_name: answer.creator_name,
            create_time: answer.create_time.to_string(),
            update_time: answer.update_time.to_string(),
            status: answer.status,
        }
    }
}

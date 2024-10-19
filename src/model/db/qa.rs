use sqlx::prelude::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum QuestionStatus{
    Active,
    Inactive,
}

impl From<&str> for QuestionStatus{
    fn from(status:&str)->Self{
        match status{
            "1"=>QuestionStatus::Active,
            "0"=>QuestionStatus::Inactive,
            _=>QuestionStatus::Inactive,
        }
    }
} 

impl Into<String> for QuestionStatus{
    fn into(self) -> String {
        match self {
            QuestionStatus::Active => "1".to_string(),
            QuestionStatus::Inactive => "0".to_string(),
        }
    }
}

#[derive(FromRow,Clone,Debug)]
#[allow(dead_code)]
pub struct Question{
    pub id:Option<i32>,
    pub sku_code:Option<String>,
    pub product_code:String,
    pub question_code:String,
    pub question_content:String,
    pub create_user_id:String,
    pub creator_name:String,
    pub create_time:DateTime<Utc>,
    pub update_time:DateTime<Utc>,
    pub status:String,
    pub sort:i32,
    pub rank:i32,
}

impl Question{
    pub fn new(sku_code:Option<String>,product_code:String,question_content:String,create_user_id:String,creator_name:String,rank:i32)->Self{
        let now = Utc::now();
        let uuid = Uuid::new_v4();
        // 将 UUID 转换为字符串形式
        let question_code = uuid.to_string();
        let question = Question{
            id:None,
            sku_code,
            product_code,
            question_code:question_code,
            question_content,
            create_user_id,
            creator_name,
            create_time:now,
            update_time:now,
            status:QuestionStatus::Active.into(),
            sort:0,
            rank,
        };
        return question;
    }
}

pub enum AnswerStatus{
    Active,
    Inactive,
}

impl From<&str> for AnswerStatus{
    fn from(status:&str)->Self{
        match status{
            "1"=>AnswerStatus::Active,
            "0"=>AnswerStatus::Inactive,
            _=>AnswerStatus::Inactive,
        }
    }
} 

impl Into<String> for AnswerStatus{
    fn into(self) -> String {
        match self {
            AnswerStatus::Active => "1".to_string(),
            AnswerStatus::Inactive => "0".to_string(),
        }
    }
}

#[derive(FromRow,Clone,Debug)]
pub struct Answer{
    pub id:Option<i32>,
    pub question_code:String,
    pub answer_content:String,
    pub create_user_id:String,
    pub create_time:DateTime<Utc>,
    pub update_time:DateTime<Utc>,
    pub status:String,
    pub creator_name:String,
}

impl Answer{
    pub fn new(question_code:String,answer_content:String,create_user_id:String,creator_name:String)->Self{
        let now = Utc::now();
        let answer = Answer{
            id:None,
            question_code,
            answer_content,
            create_user_id,
            create_time:now,
            update_time:now,
            status:AnswerStatus::Active.into(),
            creator_name,
        };
        return answer;
    }
}
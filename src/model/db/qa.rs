use sqlx::prelude::FromRow;
use crate::utils::datetime::now_local;
use uuid::Uuid;

pub enum QuestionStatus{
    Active,
    Inactive,
}

impl From<&i8> for QuestionStatus{
    fn from(status:&i8)->Self{
        match status{
            1=>QuestionStatus::Active,
            0=>QuestionStatus::Inactive,
            _=>QuestionStatus::Inactive,
        }
    }
} 

impl Into<i8> for QuestionStatus{
    fn into(self) -> i8 {
        match self {
            QuestionStatus::Active => 1,
            QuestionStatus::Inactive => 0,
        }
    }
}

#[derive(FromRow,Clone,Debug, PartialEq)]
#[allow(dead_code)]
pub struct Question{
    pub id:Option<i64>,
    pub sku_code:Option<String>,
    pub product_code:String,
    pub question_code:String,
    pub question_content:String,
    pub create_user_id:Option<String>,
    pub creator_name:Option<String>,
    pub create_time:chrono::NaiveDateTime,
    pub update_time:chrono::NaiveDateTime,
    pub status:i8,
    pub sort:i32,
    pub rank:i32,
}

impl Question{
    pub fn new(sku_code:Option<String>,product_code:String,question_content:String,create_user_id:Option<String>,
        creator_name:Option<String>,rank:i32)->Self{
        let now = now_local();
        let uuid = Uuid::new_v4();
        // 将 UUID 转换为字符串形式
        let question_code = uuid.to_string();
        let question = Question{
            id:None,
            sku_code,
            product_code,
            question_code,
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

    pub fn with_question_code(question_code:String,sku_code:Option<String>,product_code:String,question_content:String,create_user_id:Option<String>,
        creator_name:Option<String>,rank:i32)->Self{
        let now = now_local();
        // 将 UUID 转换为字符串形式
        let question = Question{
            id:None,
            sku_code,
            product_code,
            question_code,
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

    pub fn update_time(&mut self, new_date_time:chrono::NaiveDateTime){
        self.update_time = new_date_time;
    }
}

pub enum AnswerStatus{
    Active,
    Inactive,
}

impl From<&i8> for AnswerStatus{
    fn from(status:&i8)->Self{
        match status{
            1=>AnswerStatus::Active,
            0=>AnswerStatus::Inactive,
            _=>AnswerStatus::Inactive,
        }
    }
} 

impl Into<i8> for AnswerStatus{
    fn into(self) -> i8 {
        match self {
            AnswerStatus::Active => 1,
            AnswerStatus::Inactive => 0,
        }
    }
}

#[derive(FromRow,Clone,Debug, PartialEq)]
pub struct Answer{
    pub id:Option<i64>,
    pub question_code:String,
    pub answer_content:String,
    pub create_user_id:Option<String>,
    pub create_time:chrono::NaiveDateTime,
    pub update_time:chrono::NaiveDateTime,
    pub status:i8,
    pub creator_name:Option<String>,
}

impl Answer{
    pub fn new(question_code:String,answer_content:String,create_user_id:Option<String>,creator_name:Option<String>)->Self{
        let now = now_local();
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

#[derive(Clone,Debug, PartialEq)]
pub struct Page<T>{
    pub total_records:i64,
    pub total_pages:i64,
    pub current_pageno:i64,
    pub next_pageno:Option<i64>,
    pub previous_pageno:Option<i64>,
    pub page_size:i64,
    pub data:Vec<T>,
}

impl<T> Page<T>{
    pub fn new(total_records:i64,current_pageno:i64,page_size:i64,data:Vec<T>)->Self{
        let next_pageno = if total_records > current_pageno*page_size{
            Some(current_pageno+1)
        }else{
            None
        };
        let previous_pageno = if current_pageno>1{
            Some(current_pageno-1)
        }else{
            None
        };
        let total_pages = total_records/page_size;
        Page{
            total_records,
            total_pages,
            current_pageno,
            next_pageno,
            previous_pageno,
            page_size,
            data,
        }
    }
}
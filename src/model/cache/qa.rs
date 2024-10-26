use crate::model::db::qa::Question as DbQuestion;
use crate::model::db::qa::Answer as DbAnswer;
#[derive(Clone,Debug)]
pub struct Question{
    pub id:i64,
    pub product_code:String,
    pub question_code:String,
    pub question_content:String,
    pub create_user_id:Option<String>,
    pub create_time:chrono::NaiveDateTime,
    pub update_time:chrono::NaiveDateTime,
    pub status:i8,
    pub sort:i32,
    pub rank:i32,
    pub creator_name:Option<String>,
    pub answer_list:Vec<Answer>,
}

#[derive(Clone,Debug)]
pub struct Answer{
    pub id:i64,
    pub question_code:String,
    pub answer_content:String,
    pub create_user_id:Option<String>,
    pub create_time:chrono::NaiveDateTime,
    pub update_time:chrono::NaiveDateTime,
    pub status:i8,
    pub creator_name:Option<String>,
}

impl Question {
    pub fn from_db(
        db_question: DbQuestion,
        db_review_list: Vec<DbAnswer>,
    )->Self{
        Self{
            id:db_question.id.unwrap_or(0),
            product_code:db_question.product_code,
            question_code:db_question.question_code,
            question_content:db_question.question_content,
            create_user_id:db_question.create_user_id,
            creator_name:db_question.creator_name,
            create_time:db_question.create_time,
            update_time:db_question.update_time,
            status:db_question.status,
            sort:db_question.sort,
            rank:db_question.rank,
            answer_list:db_review_list.into_iter().map(|answer|Answer::from_db(answer)).collect(),
        }
    }
}

impl Answer{
    pub fn from_db(db_answer:DbAnswer)->Self{
        Self{
            id:db_answer.id.unwrap_or(0),
            question_code:db_answer.question_code,
            answer_content:db_answer.answer_content,
            create_user_id:db_answer.create_user_id,
            creator_name:db_answer.creator_name,
            create_time:db_answer.create_time,
            update_time:db_answer.update_time,
            status:db_answer.status,
        }
    }
}
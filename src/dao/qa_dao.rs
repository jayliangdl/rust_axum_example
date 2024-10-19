use crate::model::db::qa::{Question,Answer};
use axum::http::StatusCode;
pub struct QuestionDao;
impl QuestionDao{

    pub async fn insert_question(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question: &Question,
    )->Result<(), (StatusCode, String)>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!(
       "insert into rc_qa_question (
            `sku_code`,
            `product_code`,
            `question_content`,
            `create_user_id`,
            `question_code`,
            `creator_name`,
            `create_time`,
            `update_time`,
            `sort`,
            `rank`)
            values (
            ?,?,?,?,?,?,?,?,?,?
            )",
            question.sku_code.clone().unwrap_or_else(|| "".to_string()),
            question.product_code,
            question.question_content,
            question.create_user_id,
            question.question_code,
            question.creator_name,
            question.create_time,
            question.update_time,
            question.sort,
            question.rank
        );
        
        query.execute(&mut **transaction)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "插入问题失败".to_string()))?;

        Ok(())
    }

    pub async fn insert_answer(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        answer: &Answer,
    )->Result<(), (StatusCode, String)>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!(
       "insert into rc_qa_answer (
        `answer_content`,
        `create_user_id`,
        `creator_name`,
        `question_code`,
        `status`,
        `create_time`,
        `update_time`
         )
         values (
         ?,
         ?,
         ?,
         ?,
         ?,
         ?,
         ?
         )",
         answer.answer_content,
         answer.create_user_id,
         answer.creator_name,
         answer.question_code,
         answer.status,
         answer.create_time,
         answer.update_time
        );
        
        query.execute(&mut **transaction)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "插入问题失败".to_string()))?;

        Ok(())
    }

}
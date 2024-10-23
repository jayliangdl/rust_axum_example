use crate::model::request::operation::find_question_list_for_trad::FindQuestionListForTrad as RequestFindQuestionListForTrad;
use crate::model::db::qa::Question;
use crate::model::db::qa::Answer;
use axum::http::StatusCode;
use sqlx::MySqlPool;
use sqlx::QueryBuilder;
pub struct QuestionDao;
impl QuestionDao{

    //插入问题，返回问题的id
    pub async fn insert_question(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question: &Question,
    )->Result<(u64), (StatusCode, String)>{
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
        
        let result = query.execute(&mut **transaction)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "插入问题失败".to_string()))?;
        let last_insert_id = result.last_insert_id();

        Ok((last_insert_id))
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

    pub async fn delete_answer(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), (StatusCode, String)>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("delete from rc_qa_answer where `question_code` = ?",question_code);
        
        query.execute(&mut **transaction)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "依据问题删除回答失败".to_string()))?;

        Ok(())
    }

    pub async fn update_question(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question: &Question,
    )->Result<(), (StatusCode, String)>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("update rc_qa_question set `rank`= ?,`question_content` = ?,`update_time` = ?,`product_code`=? where `question_code` = ?",
         question.rank,
         question.question_content,
         question.update_time,
        question.product_code,
        question.question_code
        );
        
        query.execute(&mut **transaction)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "更新问题失败".to_string()))?;

        Ok(())
    }

    //依据question_code查询问题
    pub async fn query_question_by_question_code(
        pool: &MySqlPool,
        question_code: &String,
    )->Result<Option<Question>, (StatusCode, String)>{
        // 执行查询操作，并忽略返回的结果
        let question = sqlx::query_as::<_,Question>(
            "select * from rc_qa_question where `question_code` = ?",   
        ).bind(question_code)         
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("查询问题失败: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "查询问题失败".to_string())
        })?;
        Ok(question)
    }

    pub async fn delete_question_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), (StatusCode, String)>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("delete from rc_qa_question where `question_code` = ?",question_code);
        
        query.execute(&mut **transaction)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "删除问题失败".to_string()))?;

        Ok(())
    }

    pub async fn delete_answer_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), (StatusCode, String)>{
    
        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("delete from rc_qa_answer where `question_code` = ?",question_code);
        
        query.execute(&mut **transaction)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "删除回答失败".to_string()))?;

        Ok(())
    }

    pub async fn delete_question_and_answer_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), (StatusCode, String)>{
        let _ = QuestionDao::delete_question_by_question_code(transaction, question_code).await
        .map_err(|_| "删除Answer失败".to_string());
        let _ = QuestionDao::delete_answer_by_question_code(transaction, question_code).await
        .map_err(|_| "删除Question失败".to_string());
        Ok(())
    }


    //依据查询条件查询问题列表
    pub async fn query_question_list(
        pool: &MySqlPool,
        request_find_question_list_for_trad: &RequestFindQuestionListForTrad,        
    )->Result<Vec<Question>, (StatusCode, String)>{
        tracing::info!("request_find_question_list_for_trad: {:?}", request_find_question_list_for_trad);
        let mut builder = QueryBuilder::<sqlx::MySql>::new("select * from rc_qa_question where 1=1 ");
        if let Some(product_code) = &request_find_question_list_for_trad.product_code {
            builder.push(" and product_code like ").push_bind(product_code);
        }
        if let Some(question_content) = &request_find_question_list_for_trad.question_content {
            builder.push(" and question_content like ").push_bind(question_content);
        }

        if let Some(question_code) = &request_find_question_list_for_trad.question_code {
            builder.push(" and question_code like ").push_bind(question_code);
        }

        if let Some(create_name) = &request_find_question_list_for_trad.create_name {
            builder.push(" and create_name like ").push_bind(create_name);
        }

        if let Some(start_time) = &request_find_question_list_for_trad.start_time {
            builder.push(" and create_time >= ").push_bind(start_time);
        }
        if let Some(end_time) = &request_find_question_list_for_trad.end_time {
            builder.push(" and create_time <= ").push_bind(end_time);
        }
        if let Some(answer_content) = &request_find_question_list_for_trad.answer_content {
            builder.push(" and question_code  in (select question_code from rc_qa_answer where status=1 and answer_content like ").push_bind(answer_content).push(")");
        }

        builder.push(" order by `sort` desc,`rank`,`create_time` desc,`id`");

        //执行查询操作，并忽略返回的结果


        let sql = builder.sql();
        tracing::info!("sql: {}", sql);
        let mut query: sqlx::query::QueryAs<'_, sqlx::MySql, Question, sqlx::mysql::MySqlArguments> = sqlx::query_as::<_, Question>(sql);
        if let Some(product_code) = &request_find_question_list_for_trad.product_code {
            query = query.bind(format!("%{}%",product_code));
        }
        if let Some(question_content) = &request_find_question_list_for_trad.question_content {
            query = query.bind(format!("%{}%",question_content));
        }

        if let Some(question_code) = &request_find_question_list_for_trad.question_code {
            query = query.bind(format!("%{}%",question_code));
        }

        if let Some(create_name) = &request_find_question_list_for_trad.create_name {
            query = query.bind(format!("%{}%",create_name));
        }

        if let Some(start_time) = &request_find_question_list_for_trad.start_time {
            query = query.bind(format!("{}",start_time));
        }
        if let Some(end_time) = &request_find_question_list_for_trad.end_time {
            query = query.bind(format!("{}",end_time));
        }
        if let Some(answer_content) = &request_find_question_list_for_trad.answer_content {
            query = query.bind(format!("{}",answer_content));
        }
        let result = query.fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库错误： {}", e)))?;
        Ok(result)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::model::request::operation::find_question_list_for_trad::FindQuestionListForTrad as RequestFindQuestionListForTrad;
    use crate::model::db::qa::Question;
    use crate::dao::qa_dao::QuestionDao;
    use crate::utils::db::init_pool;
    use crate::utils::logging::init_log;
    use chrono::{NaiveDateTime,NaiveDate,NaiveTime};

    async fn delete_test_data(pool: &MySqlPool,question_code:&String){
        let mut transaction = pool.begin().await
        .map_err(|_| "Failed to start transaction".to_string()).unwrap();
        let _ = QuestionDao::delete_question_and_answer_by_question_code(&mut transaction, question_code).await
        .map_err(|_| "删除Answer失败".to_string());
        transaction.commit().await
        .map_err(|_| "Failed to commit transaction".to_string()).unwrap();
    }   
    async fn prepare_data(pool: &MySqlPool)->Result<(Question,),String>{
        init_log().await; // 日志初始化
        tracing::info!("prepare_data start");
        let d = NaiveDate::from_ymd_opt(2014, 10, 22).unwrap();
        let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 000).unwrap();
        let dt = NaiveDateTime::new(d, t);

        let mut transaction = pool.begin().await
        .map_err(|_| "Failed to start transaction".to_string())?;
        let sku_code = Some("sku_code".to_string());
        let product_code = "product_code".to_string();
        let question_content = "question_content".to_string();
        let create_user_id = Some("create_user_id".to_string());
        let question_code = "question_code".to_string();
        let creator_name = Some("creator_name".to_string());
        let sort = 1;
        let rank = 1;
        let mut question = Question::new(sku_code,product_code,question_content,create_user_id,
            creator_name,rank);
        question.question_code = question_code.clone();
        question.create_time = dt;
        question.update_time = dt;
        question.sort = sort;
    
        let question_id = QuestionDao::insert_question(&mut transaction, &question).await
        .map_err(|_| "插入Question到数据库失败".to_string());
        

        let answer_content = "answer_content".to_string();
        let create_user_id = Some("create_user_id".to_string());
        let creator_name = Some("creator_name".to_string());
        //question_code:String,answer_content:String,create_user_id:Option<String>,creator_name:Option<String>
        let mut answer = Answer::new(question_code,answer_content,create_user_id,creator_name);
        answer.create_time = dt;
        answer.update_time = dt;
        let _ = QuestionDao::insert_answer(&mut transaction, &answer).await
        .map_err(|_| "插入Question到数据库失败".to_string());
        
        // 提交事务
        transaction.commit().await
        .map_err(|_| "Failed to commit transaction".to_string())?;

        question.id=Some(question_id.unwrap() as i64);
        Ok((question,))
    }

    //循环对比actual_questions和expected_questions，仅对比question_code，question_content，sku_code，create_user_id,creator_name,sort,rank
    fn comare_question(actual_questions: &Vec<&Question>,expected_questions: &Vec<&Question>){
        assert_eq!(actual_questions.len(),expected_questions.len());
        for i in 0..actual_questions.len(){
            let actual_question = &actual_questions[i];
            let expected_question = &expected_questions[i];
            assert_eq!(actual_question.question_code,expected_question.question_code);
            assert_eq!(actual_question.question_content,expected_question.question_content);
            assert_eq!(actual_question.sku_code,expected_question.sku_code);
            assert_eq!(actual_question.create_user_id,expected_question.create_user_id);
            assert_eq!(actual_question.creator_name,expected_question.creator_name);
            assert_eq!(actual_question.sort,expected_question.sort);
            assert_eq!(actual_question.rank,expected_question.rank);
        }
    }

    #[tokio::test]  
    async fn test_query_question_list() {
        init_log().await; // 日志初始化
        let pool: sqlx::Pool<sqlx::MySql> = init_pool().await.expect("Cannot init the database pool");
        let expected_value = prepare_data(&pool).await.expect("prepare data error");
        let expected_question = expected_value.0;
        tracing::info!("test_query_question_list start");
        tracing::info!("pool: {:?}", pool);
        // //模拟查询条件
        let mut request = RequestFindQuestionListForTrad::new();
        request.product_code=Some("product_code".to_string());
        //模拟查询
        let result: Result<Vec<Question>, (StatusCode, String)> = QuestionDao::query_question_list(&pool,&request).await;
        tracing::info!("result: {:?}", result);
        assert_eq!(result.is_ok(),true);
        let unwrap_questions = result.unwrap();
        let actual_questions = unwrap_questions.iter().collect();
        let expected_questions = vec![&expected_question];
        comare_question(&actual_questions,&expected_questions);  
        delete_test_data(&pool,&expected_question.question_code).await;
    }   
}

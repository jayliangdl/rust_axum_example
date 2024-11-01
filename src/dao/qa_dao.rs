use crate::model::request::operation::find_question_list_for_trad::FindQuestionListForTrad as RequestFindQuestionListForTrad;
use crate::model::db::qa::Question;
use crate::model::db::qa::Answer;
use crate::model::db::qa::Page;
use crate::utils::error::BusinessError;
use sqlx::MySqlPool;
use sqlx::QueryBuilder;
pub struct QuestionDao;
impl QuestionDao{

    /// 插入问题，返回问题的id
    pub async fn insert_question(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question: &Question,
    )->Result<u64, BusinessError>{
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
        .await?;
        let last_insert_id = result.last_insert_id();

        Ok(last_insert_id)
    }

    pub async fn insert_answer(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        answer: &Answer,
    )->Result<(), BusinessError>{

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
        .await?;

        Ok(())
    }

    pub async fn delete_answer(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), BusinessError>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("delete from rc_qa_answer where `question_code` = ?",question_code);
        
        query.execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn update_question(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question: &Question,
    )->Result<(), BusinessError>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("update rc_qa_question set `rank`= ?,`question_content` = ?,`update_time` = ?,`product_code`=? where `question_code` = ?",
         question.rank,
         question.question_content,
         question.update_time,
        question.product_code,
        question.question_code
        );
        
        query.execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// 依据question_code查询有效的问题记录
    pub async fn find_question_by_question_code(
        pool: &MySqlPool,
        question_code: &String,
    )->Result<Option<Question>, BusinessError>{
        // 执行查询操作，并忽略返回的结果
        let question = sqlx::query_as::<_,Question>(
            "select * from rc_qa_question where `question_code` = ? and status='1'",   
        ).bind(question_code)         
        .fetch_optional(pool)
        .await?;
        Ok(question)
    }

    /// 依据question_code删除问题记录
    pub async fn delete_question_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), BusinessError>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("delete from rc_qa_question where `question_code` = ?",question_code);
        
        query.execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// 依据question_code删除回答记录
    pub async fn delete_answer_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), BusinessError>{
    
        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("delete from rc_qa_answer where `question_code` = ?",question_code);
        
        query.execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn delete_question_and_answer_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), BusinessError>{
        let _ = QuestionDao::delete_question_by_question_code(transaction, question_code).await?;
        // .map_err(|_| "删除Answer失败".to_string());
        let _ = QuestionDao::delete_answer_by_question_code(transaction, question_code).await?;
        // .map_err(|_| "删除Question失败".to_string());
        Ok(())
    }

    /// 依据question_code软删除问题记录（设置失效）
    pub async fn disabled_question_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), BusinessError>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("update rc_qa_question set `status`='0' 
        where `question_code` = ? and `status`='1'",question_code);
        
        query.execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// 依据question_code软删除回答记录（设置失效）
    pub async fn disable_answer_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), BusinessError>{
    
        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("update rc_qa_answer set `status`='0' where `question_code` = ? and `status`='1'",question_code);
        
        query.execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn disabled_question_and_answer_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code: &String,
    )->Result<(), BusinessError>{
        let _ = QuestionDao::disable_answer_by_question_code(transaction, question_code).await?;
        let _ = QuestionDao::disabled_question_by_question_code(transaction, question_code).await?;
        Ok(())
    }

    //构造查询条件
    pub fn query_question_list_condition<'a>(
        builder:&mut QueryBuilder<'a, sqlx::MySql>,
        request_find_question_list_for_trad: &'a RequestFindQuestionListForTrad
    ){
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
    }
    
    pub async fn query_question_list(
        pool: &MySqlPool,
        request_find_question_list_for_trad: &RequestFindQuestionListForTrad, 
        current_pageno:i64,
        page_size:i64,    
    )->Result<Page<Question>, BusinessError>{
        let total_records = Self::query_question_count(pool,request_find_question_list_for_trad).await?;
        let questions = Self::query_question_list_by_page(pool,request_find_question_list_for_trad,current_pageno,page_size).await?;
        let page = Page::new(total_records, current_pageno, page_size, questions);
        return Ok(page);
    }

    /// 依据查询条件查询问题列表（分页）
    async fn query_question_list_by_page(
        pool: &MySqlPool,
        request_find_question_list_for_trad: &RequestFindQuestionListForTrad, 
        current_pageno:i64,
        page_size:i64       
    )->Result<Vec<Question>, BusinessError>{
        tracing::info!("request_find_question_list_for_trad: {:?}", request_find_question_list_for_trad);
        let mut builder: QueryBuilder<'_, sqlx::MySql> = QueryBuilder::<sqlx::MySql>::new("select * from rc_qa_question where 1=1 ");
        Self::query_question_list_condition(&mut builder,request_find_question_list_for_trad);
        let offset = (current_pageno-1)*page_size;
        builder.push(format!(" order by `sort` desc,`rank`,`create_time` desc,`id` limit {} offset {}",page_size,offset));
        let sql = builder.sql();
        tracing::info!("sql: {:?}", sql);
        let query = builder.build_query_as::<Question>();
        let result = query.fetch_all(pool)
            .await?;
        Ok(result)
    }

    /// 依据查询条件查询问题列表
    async fn query_question_count(
        pool: &MySqlPool,
        request_find_question_list_for_trad: &RequestFindQuestionListForTrad,        
    )->Result<i64,BusinessError>{
        tracing::info!("request_find_question_list_for_trad: {:?}", request_find_question_list_for_trad);
        let mut builder: QueryBuilder<'_, sqlx::MySql> = QueryBuilder::<sqlx::MySql>::new("select count(1) as total_count from rc_qa_question where 1=1 ");
        Self::query_question_list_condition(&mut builder,request_find_question_list_for_trad);

        let query = builder.build_query_as::<(i64,)>();
        let result = query.fetch_one(pool)
            .await?;          
        Ok(result.0)
    }

    /// 依据question_code查询回答记录
    pub async fn query_answer_by_question_code(
        pool: &MySqlPool,
        question_code: &String,
    )->Result<Vec<Answer>, BusinessError>{
        let answer = sqlx::query_as::<_,Answer>(
            "select * from rc_qa_answer where `question_code` = ?",   
        ).bind(question_code)         
        .fetch_all(pool)
        .await?;
        Ok(answer)
    }

    pub async fn update_sort_by_question_code(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        question_code:&String,
        sort:i32
    )->Result<(), BusinessError>{
        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!("update rc_qa_question set sort=? where question_code = ? and status=1",sort,question_code);
        
        query.execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// 查询下一个sort值
    /// 逻辑：查询数据库中当前最大sort值+1
    pub async fn have_next_sort(
        pool: &MySqlPool,      
    )->Result<i32, BusinessError>{
        let mut builder: QueryBuilder<'_, sqlx::MySql> = QueryBuilder::<sqlx::MySql>::new("select max(sort)+1 as max_sort from rc_qa_question where 1=1 and status=1 ");
        let query = builder.build_query_as::<(i32,)>();
        let result = query.fetch_one(pool)
            .await?;
        Ok(result.0)
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
    fn compare_question(actual_questions: &Vec<Question>,expected_questions: &Vec<Question>){
        assert!(actual_questions.len()==expected_questions.len());
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
    async fn test_query_question_list(){
        init_log().await; // 日志初始化
        let pool: sqlx::Pool<sqlx::MySql> = init_pool().await.expect("Cannot init the database pool");
        let expected_value = prepare_data(&pool).await.expect("prepare data error");
        let expected_question = expected_value.0;

        tracing::info!("test_query_question_list start");
        tracing::info!("pool: {:?}", pool);

        let mut request = RequestFindQuestionListForTrad::new();
        request.product_code=Some("product_code".to_string());
        //模拟查询
        let current_pageno = 1;
        let page_size = 10;
        let result: Result<Vec<Question>, BusinessError> = QuestionDao::query_question_list_by_page(&pool,&request,current_pageno,page_size).await;
        tracing::info!("result: {:?}", result);
        let result = std::panic::catch_unwind(||{
            assert_eq!(result.is_ok(),true);
            let unwrap_questions = result.unwrap();
            let actual_questions = unwrap_questions;
            let mut expected_questions = Vec::new();
            expected_questions.push(expected_question.clone());
            compare_question(&actual_questions,&expected_questions);
        });
        
        match result {
            Ok(_) => {
                delete_test_data(&pool, &expected_question.question_code).await;
                println!("没有发生 panic")
            },
            Err(e) => {
                delete_test_data(&pool, &expected_question.question_code).await;
                if let Some(payload) = e.downcast_ref::<&str>() {
                    println!("发生了带有字符串消息的 panic: {}", payload);
                } else {
                    println!("发生了 panic，但没有具体的错误信息");
                }
            },
        }
    }

    async fn prepare_data1(pool: &MySqlPool)->Result<(Vec<Question>,),String>{
        init_log().await; // 日志初始化
        tracing::info!("prepare_data1 start");
        let d = NaiveDate::from_ymd_opt(2014, 10, 22).unwrap();
        let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 000).unwrap();
        let dt = NaiveDateTime::new(d, t);

        let mut transaction = pool.begin().await
        .map_err(|_| "Failed to start transaction".to_string())?;

        let mut questions = Vec::new();
        for i in 0..55{
            let sku_code = Some("sku_code".to_string());
            let product_code = format!("product_code_{}",i);
            let question_content = "question_content".to_string();
            let create_user_id = Some("create_user_id".to_string());
            let question_code = format!("question_code_{}",i);
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

            question.id=Some(question_id.unwrap() as i64);
            questions.push(question);
        }

        // 提交事务
        transaction.commit().await
        .map_err(|_| "Failed to commit transaction".to_string())?;

        
        Ok((questions,))
    }

    async fn delete_test_data1(pool: &MySqlPool,question_codes:&Vec<String>){
        let mut transaction = pool.begin().await
            .map_err(|_| "Failed to start transaction".to_string()).unwrap();
        for i in 0..question_codes.len(){
            let _ = QuestionDao::delete_question_and_answer_by_question_code(&mut transaction, &question_codes[i]).await
            .map_err(|_| "删除Answer失败".to_string());
        }
        transaction.commit().await
        .map_err(|_| "Failed to commit transaction".to_string()).unwrap();
    }

    

    #[tokio::test]
    async fn test_query_question_list1(){
        init_log().await; // 日志初始化
        let pool: sqlx::Pool<sqlx::MySql> = init_pool().await.expect("Cannot init the database pool");
        let expected_value = prepare_data1(&pool).await.expect("prepare data error");
        let expected_questions = expected_value.0;

        tracing::info!("test_query_question_list1 start");
        tracing::info!("pool: {:?}", pool);

        let request = RequestFindQuestionListForTrad::new();
        // request.product_code=Some("product_code".to_string());
        //模拟查询
        let current_pageno = 2;
        let page_size = 10;
        let result: Result<Page<Question>, BusinessError> = QuestionDao::query_question_list(&pool,&request,current_pageno,page_size).await;
        if let Err(e) = &result{
            tracing::error!("result: {:?}", e);
        }
        if let Ok(page) = &result{
            tracing::info!("page.total_records: {:?}", page.total_records);
            tracing::info!("page.total_pages: {:?}", page.total_pages);
            tracing::info!("page.current_pageno: {:?}", page.current_pageno);
            tracing::info!("page.next_pageno: {:?}", page.next_pageno);
            tracing::info!("page.previous_pageno: {:?}", page.previous_pageno);
            tracing::info!("page.page_size: {:?}", page.page_size);
        }
        let result = std::panic::catch_unwind(||{
            assert_eq!(result.is_ok(),true);

            let unwrap_questions = result.unwrap();
            let actual_questions = unwrap_questions.data.into_iter().collect();
            tracing::info!("actual_questions:{:?}",&actual_questions); 
            // let actual_questions = unwrap_questions.iter().collect();
            let expected_questions = expected_questions[10..20].to_vec().into_iter().collect();
            tracing::info!("expected_questions:{:?}",&expected_questions); 
            compare_question(&actual_questions,&expected_questions);
        });
        
        match result {
            Ok(_) => {
                let question_codes = expected_questions.iter().map(|x|x.question_code.clone()).collect();    
                delete_test_data1(&pool, &question_codes).await;
                println!("没有发生 panic")
            },
            Err(e) => {
                let question_codes = expected_questions.iter().map(|x|x.question_code.clone()).collect();  
                delete_test_data1(&pool, &question_codes).await;
                if let Some(payload) = e.downcast_ref::<&str>() {
                    println!("发生了带有字符串消息的 panic: {}", payload);
                } else {
                    println!("发生了 panic，但没有具体的错误信息");
                }
            },
        }
    }
}

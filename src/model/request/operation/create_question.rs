
use serde::{Serialize,Deserialize};
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{Validate, ValidationError};
use crate::model::db::qa::{
    Answer as DbAnswer,
    Question as DbQuestion
};
use crate::utils::error::BusinessError;
use crate::utils::default_value::deserialize_null_to_empty_string;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct CreateQuestion {
    pub sku_code: Option<String>,
    #[validate(length(min = 1, message = "productCode不能为空"))]
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[validate(length(min = 1, message = "questionContent不能为空"))]
    #[serde(rename = "questionContent")]
    pub question_content: String,
    #[validate(length(min = 1, message = "createUserId不能为空"))]
    #[serde(rename = "createUserId")]
    pub create_user_id: String,
    
    #[serde(rename = "creatorName", deserialize_with = "deserialize_null_to_empty_string")]
    pub creator_name: String,
    pub rank: i32,
    #[validate(custom = "validate_answer_list")]
    #[serde(rename = "answers")]
    pub answer_list: Vec<CreateAnswer>,
}

impl CreateQuestion{
    pub fn into_db_question(&self)->DbQuestion{
        let mut db_answer_list = vec![];
        let db_question: DbQuestion = DbQuestion::new(
            self.sku_code.clone(), 
            self.product_code.clone(), 
            self.question_content.clone(), 
            Some(self.create_user_id.clone()), 
            Some(self.creator_name.clone()), 
            self.rank
        );
        for answer in &self.answer_list{
            let db_answer = answer.into_db_answer(db_question.question_code.clone());
            db_answer_list.push(db_answer);
        };
        db_question
    }

    pub async fn custom_validate(&self)->Result<(), BusinessError>{
        self.validate()?;
        for answer in &self.answer_list{
            let _ = answer.validate()?;
        }        
        Ok(())
    }
}


impl CreateAnswer{
    pub fn into_db_answer(&self,question_code:String)->DbAnswer{
        let db_answer: DbAnswer = DbAnswer::new(
            question_code, 
            self.answer_content.clone(), 
            Some(self.create_user_id.clone()), 
            Some(self.creator_name.clone()),
        );
        db_answer
    }
}

#[derive(Serialize, Deserialize, Debug, Validate,Clone)]
pub struct CreateAnswer{
    #[validate(length(min = 1, message = "answerContent不能为空"))]
    #[serde(rename = "answerContent")]
    pub answer_content:String,
    #[validate(length(min = 1, message = "createUserId不能为空"))]
    #[serde(rename = "createUserId")]
    pub create_user_id: String,
    #[serde(rename = "creatorName", deserialize_with = "deserialize_null_to_empty_string")]    
    pub creator_name: String,
}

fn validate_answer_list(value: &Vec<CreateAnswer>) -> Result<(), ValidationError> {
    if value.len() == 0 {
        return Err(ValidationError{code:"".into(),message:Some(Cow::from("answers列表不能为空".to_string())),params:HashMap::new()});
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::model::request::operation::create_question::CreateQuestion;
    use crate::model::request::operation::create_question::CreateAnswer;
    use crate::utils::error::BusinessError;
    use crate::utils::logging::init_log;

    //验证参数校验异常情况的总体情况
    #[tokio::test]
    async fn test_validate_error() {
        init_log().await;
        let error = BusinessError::InvalidParameter((None, None));
        assert_eq!(error.code(), "1399001");
        assert_eq!(error.msg(), "参数校验异常");
    }
    //模拟product_code为空的情况
    #[tokio::test]
    async fn test_validate_product_code_empty() {
        init_log().await;
        let sku_code = Some("sku_code".to_string());
        let product_code = "".to_string();
        let question_content = "question_content".to_string();
        let create_user_id = "create_user_id".to_string();
        let creator_name = "creator_name".to_string();
        let rank = 1;
        let answer_list = vec![
            CreateAnswer {
                answer_content: "answer_content".to_string(),
                create_user_id: "create_user_id".to_string(),
                creator_name: "creator_name".to_string(),
            },
        ];
        let question = CreateQuestion {
            sku_code,
            product_code,
            question_content,
            create_user_id,
            creator_name,
            rank,
            answer_list,
        };
        let result = question.custom_validate().await;
        assert_eq!(result.is_err(), true);
        if let Err(error) = result {
            match error {
                BusinessError::InvalidParameter ((_,errors_parameters)) => {
                    assert_eq!(errors_parameters.is_some(), true);
                    let error_parameters = errors_parameters.unwrap();
                    let error_parameters0 = error_parameters.get(0);
                    assert_eq!(error_parameters0.is_some(), true);
                    let error_parameters0 = error_parameters0.unwrap();
                    assert_eq!(error_parameters0.contains_key("field"), true);
                    assert_eq!(error_parameters0.contains_key("message"), true);
                    assert_eq!(error_parameters0.get("field").unwrap(),"productCode");  
                    assert_eq!(error_parameters0.get("message").unwrap(),"productCode不能为空");  
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    //模拟question_content为空的情况
    #[tokio::test]
    async fn test_validate_question_content_empty() {
        init_log().await;

        let sku_code = Some("sku_code".to_string());
        let product_code = "product_code".to_string();
        let question_content = "".to_string();
        let create_user_id = "create_user_id".to_string();
        let creator_name = "creator_name".to_string();
        let rank = 1;
        let answer_list = vec![
            CreateAnswer {
                answer_content: "answer_content".to_string(),
                create_user_id: "create_user_id".to_string(),
                creator_name: "creator_name".to_string(),
            },
        ];
        let question = CreateQuestion {
            sku_code,
            product_code,
            question_content,
            create_user_id,
            creator_name,
            rank,
            answer_list,
        };
        let result = question.custom_validate().await;
        assert_eq!(result.is_err(), true);
        if let Err(error) = result {
            match error {
                BusinessError::InvalidParameter ((_,errors_parameters)) => {
                    assert_eq!(errors_parameters.is_some(), true);
                    let error_parameters = errors_parameters.unwrap();
                    let error_parameters0 = error_parameters.get(0);
                    assert_eq!(error_parameters0.is_some(), true);
                    let error_parameters0 = error_parameters0.unwrap();
                    assert_eq!(error_parameters0.contains_key("field"), true);
                    assert_eq!(error_parameters0.contains_key("message"), true);
                    assert_eq!(error_parameters0.get("field").unwrap(),"questionContent");  
                    assert_eq!(error_parameters0.get("message").unwrap(),"questionContent不能为空");  
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    //模拟create_user_id为空的情况
    #[tokio::test]
    async fn test_validate_create_user_id_empty() {
        init_log().await;
        let sku_code = Some("sku_code".to_string());
        let product_code = "product_code".to_string();
        let question_content = "question_content".to_string();
        let create_user_id = "".to_string();
        let creator_name = "creator_name".to_string();
        let rank = 1;
        let answer_list = vec![
            CreateAnswer {
                answer_content: "answer_content".to_string(),
                create_user_id: "create_user_id".to_string(),
                creator_name: "creator_name".to_string(),
            },
        ];
        let question = CreateQuestion {
            sku_code,
            product_code,
            question_content,
            create_user_id,
            creator_name,
            rank,
            answer_list,
        };
        let result = question.custom_validate().await;
        assert_eq!(result.is_err(), true);
        if let Err(error) = result {
            match error {
                BusinessError::InvalidParameter ((_,errors_parameters)) => {
                    assert_eq!(errors_parameters.is_some(), true);
                    let error_parameters = errors_parameters.unwrap();
                    let error_parameters0 = error_parameters.get(0);
                    assert_eq!(error_parameters0.is_some(), true);
                    let error_parameters0 = error_parameters0.unwrap();
                    assert_eq!(error_parameters0.contains_key("field"), true);
                    assert_eq!(error_parameters0.contains_key("message"), true);
                    assert_eq!(error_parameters0.get("field").unwrap(),"createUserId");  
                    assert_eq!(error_parameters0.get("message").unwrap(),"createUserId不能为空");  
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }


    //模拟answers列表为空的情况
    #[tokio::test]
    async fn test_validate_answer_list_empty() {
        init_log().await;
        let sku_code = Some("sku_code".to_string());
        let product_code = "product_code".to_string();
        let question_content = "question_content".to_string();
        let create_user_id = "create_user_id".to_string();
        let creator_name = "creator_name".to_string();
        let rank = 1;
        let answer_list = vec![];
        let question = CreateQuestion {
            sku_code,
            product_code,
            question_content,
            create_user_id,
            creator_name,
            rank,
            answer_list,
        };
        let result = question.custom_validate().await;
        assert_eq!(result.is_err(), true);
        if let Err(error) = result {
            match error {
                BusinessError::InvalidParameter ((_,errors_parameters)) => {
                    assert_eq!(errors_parameters.is_some(), true);
                    let error_parameters = errors_parameters.unwrap();
                    let error_parameters0 = error_parameters.get(0);
                    assert_eq!(error_parameters0.is_some(), true);
                    let error_parameters0 = error_parameters0.unwrap();
                    assert_eq!(error_parameters0.contains_key("field"), true);
                    assert_eq!(error_parameters0.contains_key("message"), true);
                    assert_eq!(error_parameters0.get("field").unwrap(),"answers");  
                    assert_eq!(error_parameters0.get("message").unwrap(),"answers列表不能为空");  
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    //模拟answers列表中answer_content为空的情况
    #[tokio::test]
    async fn test_validate_answer_content_empty() {
        init_log().await;
        let sku_code = Some("sku_code".to_string());
        let product_code = "product_code".to_string();
        let question_content = "question_content".to_string();
        let create_user_id = "create_user_id".to_string();
        let creator_name = "creator_name".to_string();
        let rank = 1;
        let answer_list = vec![
            CreateAnswer {
                answer_content: "".to_string(),
                create_user_id: "create_user_id".to_string(),
                creator_name: "creator_name".to_string(),
            },
        ];
        let question = CreateQuestion {
            sku_code,
            product_code,
            question_content,
            create_user_id,
            creator_name,
            rank,
            answer_list,
        };
        let result = question.custom_validate().await;
        assert_eq!(result.is_err(), true);
        if let Err(error) = result {
            match error {
                BusinessError::InvalidParameter ((_,errors_parameters)) => {
                    assert_eq!(errors_parameters.is_some(), true);
                    let error_parameters = errors_parameters.unwrap();
                    let error_parameters0 = error_parameters.get(0);
                    assert_eq!(error_parameters0.is_some(), true);
                    let error_parameters0 = error_parameters0.unwrap();
                    assert_eq!(error_parameters0.contains_key("field"), true);
                    assert_eq!(error_parameters0.contains_key("message"), true);
                    assert_eq!(error_parameters0.get("field").unwrap(),"answerContent");  
                    assert_eq!(error_parameters0.get("message").unwrap(),"answerContent不能为空");  
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    //模拟answers列表中create_user_id为空的情况

    #[tokio::test]
    async fn test_validate_create_user_id_in_answer_empty() {
        init_log().await;
        let sku_code = Some("sku_code".to_string());
        let product_code = "product_code".to_string();
        let question_content = "question_content".to_string();
        let create_user_id = "create_user_id".to_string();
        let creator_name = "creator_name".to_string();
        let rank = 1;
        let answer_list = vec![
            CreateAnswer {
                answer_content: "answer_content".to_string(),
                create_user_id: "".to_string(),
                creator_name: "creator_name".to_string(),
            },
        ];
        let question = CreateQuestion {
            sku_code,
            product_code,
            question_content,
            create_user_id,
            creator_name,
            rank,
            answer_list,
        };
        let result = question.custom_validate().await;
        assert_eq!(result.is_err(), true);
        if let Err(error) = result {
            match error {
                BusinessError::InvalidParameter ((_,errors_parameters)) => {
                    assert_eq!(errors_parameters.is_some(), true);
                    let error_parameters = errors_parameters.unwrap();
                    let error_parameters0 = error_parameters.get(0);
                    assert_eq!(error_parameters0.is_some(), true);
                    let error_parameters0 = error_parameters0.unwrap();
                    assert_eq!(error_parameters0.contains_key("field"), true);
                    assert_eq!(error_parameters0.contains_key("message"), true);
                    assert_eq!(error_parameters0.get("field").unwrap(),"createUserId");  
                    assert_eq!(error_parameters0.get("message").unwrap(),"createUserId不能为空");  
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }
}
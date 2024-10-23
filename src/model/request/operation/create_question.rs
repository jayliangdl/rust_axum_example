
use serde::{Serialize,Deserialize};
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{Validate, ValidationError};
use crate::model::db::qa::{
    Answer as DbAnswer,
    Question as DbQuestion
};
use crate::models::ApiResponse;
use crate::utils::error::ErrorCode;
use crate::model::response::operation::create_question::CreateQuestion as ResponseCreateQuestion;
use crate::utils::default_value::deserialize_null_to_empty_string;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct CreateQuestion {
    pub sku_code: Option<String>,
    #[validate(length(min = 1, message = "product_code不能为空"))]
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[validate(length(min = 1, message = "question_content不能为空"))]
    #[serde(rename = "questionContent")]
    pub question_content: String,
    #[validate(length(min = 1, message = "create_user_id不能为空"))]
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

    pub async fn custom_validate(&self)->Result<(), ApiResponse<ResponseCreateQuestion>>{
        if let Err(errors) = self.validate(){
            let e: ApiResponse<ResponseCreateQuestion> = ErrorCode::InvalidParameter.to_response_from_validation_errors::<ResponseCreateQuestion>(errors,None);
            return Err(e);
        }
        for answer in &self.answer_list{
            if let Err(errors) = answer.validate(){
                let e: ApiResponse<ResponseCreateQuestion> = ErrorCode::InvalidParameter.to_response_from_validation_errors::<ResponseCreateQuestion>(errors,None);
                return Err(e);
            }
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
    #[validate(length(min = 1, message = "answer_content不能为空"))]
    #[serde(rename = "answerContent")]
    pub answer_content:String,
    #[validate(length(min = 1, message = "create_user_id不能为空"))]
    #[serde(rename = "createUserId")]
    pub create_user_id: String,
    #[serde(rename = "creatorName", deserialize_with = "deserialize_null_to_empty_string")]    
    pub creator_name: String,
}

fn validate_answer_list(value: &Vec<CreateAnswer>) -> Result<(), ValidationError> {
    if value.len() == 0 {
        return Err(ValidationError{code:"".into(),message:Some(Cow::from("answer_list列表不能为空".to_string())),params:HashMap::new()});
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::model::request::operation::create_question::CreateQuestion;
    use crate::model::request::operation::create_question::CreateAnswer;
    use crate::models::ApiResponse;

    #[tokio::test]
    async fn test_validate_product_code_empty() {
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
                ApiResponse::ERROR { msg, error_parameters, .. } => {
                    assert_eq!(msg, "入参错误");
                    assert_eq!(
                        error_parameters.unwrap().get("productCode")
                            .unwrap().as_array()
                            .unwrap().get(0)
                            .unwrap().get("message").unwrap().as_str().unwrap(),
                        "product_code不能为空"
                    );
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    #[tokio::test]
    async fn test_validate_question_content_empty() {
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
                ApiResponse::ERROR { msg, error_parameters, .. } => {
                    assert_eq!(msg, "入参错误");
                    assert_eq!(
                        error_parameters.unwrap().get("questionContent")
                            .unwrap().as_array()
                            .unwrap().get(0)
                            .unwrap().get("message").unwrap().as_str().unwrap(),
                        "question_content不能为空"
                    );
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    #[tokio::test]
    async fn test_validate_create_user_id_empty() {
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
                ApiResponse::ERROR { msg, error_parameters, .. } => {
                    assert_eq!(msg, "入参错误");
                    assert_eq!(
                        error_parameters.unwrap().get("createUserId")
                            .unwrap().as_array()
                            .unwrap().get(0)
                            .unwrap().get("message").unwrap().as_str().unwrap(),
                        "create_user_id不能为空"
                    );
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    #[tokio::test]
    async fn test_validate_answer_list_empty() {
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
                ApiResponse::ERROR { msg, error_parameters, .. } => {
                    assert_eq!(msg, "入参错误");
                    assert_eq!(
                        error_parameters.unwrap().get("answers")
                            .unwrap().as_array()
                            .unwrap().get(0)
                            .unwrap().get("message").unwrap().as_str().unwrap(),
                        "answer_list列表不能为空"
                    );
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    #[tokio::test]
    async fn test_validate_answer_content_empty() {
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
                ApiResponse::ERROR { msg, error_parameters, .. } => {
                    assert_eq!(msg, "入参错误");
                    assert_eq!(
                        error_parameters.unwrap().get("answerContent")
                            .unwrap().as_array()
                            .unwrap().get(0)
                            .unwrap().get("message").unwrap().as_str().unwrap(),
                        "answer_content不能为空"
                    );
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

    #[tokio::test]
    async fn test_validate_answer_create_user_id_empty() {
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
                ApiResponse::ERROR { msg, error_parameters, .. } => {
                    assert_eq!(msg, "入参错误");
                    assert_eq!(
                        error_parameters.unwrap().get("createUserId")
                            .unwrap().as_array()
                            .unwrap().get(0)
                            .unwrap().get("message").unwrap().as_str().unwrap(),
                        "create_user_id不能为空"
                    );
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }
}
use serde::{Serialize,Deserialize};
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{Validate, ValidationError};
use crate::model::db::qa::{
    Answer as DbAnswer,
    Question as DbQuestion
};
use crate::utils::error::BusinessError;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct UpdateQuestion {
    pub sku_code: Option<String>,
    #[validate(length(min = 1, message = "productCode不能为空"))]
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[validate(length(min = 1, message = "questionContent不能为空"))]
    #[serde(rename = "questionContent")]
    pub question_content: String,
    pub rank: i32,
    #[validate(custom = "validate_answer_list")]
    #[serde(rename = "answers")]
    pub answer_list: Vec<UpdateAnswer>,
    #[validate(length(min = 1, message = "questionCode不能为空"))]
    #[serde(rename = "questionCode")]
    pub question_code: String,
}

impl UpdateQuestion{
    pub fn into_db_question(&self)->DbQuestion{
        let mut db_answer_list = vec![];
        let db_question: DbQuestion = DbQuestion::with_question_code(
            self.question_code.clone(),
            self.sku_code.clone(), 
            self.product_code.clone(), 
            self.question_content.clone(), 
            None,
            None,
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
            answer.validate()?;
        }        
        Ok(())
    }
}


impl UpdateAnswer{
    pub fn into_db_answer(&self,question_code:String)->DbAnswer{
        let db_answer: DbAnswer = DbAnswer::new(
            question_code, 
            self.answer_content.clone(), 
            None,
            None,
        );
        db_answer
    }
}

#[derive(Serialize, Deserialize, Debug, Validate,Clone)]
pub struct UpdateAnswer{
    #[validate(length(min = 1, message = "answerContent不能为空"))]
    #[serde(rename = "answerContent")]
    pub answer_content:String,
}

fn validate_answer_list(value: &Vec<UpdateAnswer>) -> Result<(), ValidationError> {
    if value.len() == 0 {
        return Err(ValidationError{code:"".into(),message:Some(Cow::from("answers列表不能为空".to_string())),params:HashMap::new()});
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::model::request::operation::update_question::UpdateQuestion;
    use crate::model::request::operation::update_question::UpdateAnswer;
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
        let question_code = "question_code".to_string();
        let product_code = "".to_string();
        let question_content = "question_content".to_string();
        let rank = 1;
        let answer_list = vec![
            UpdateAnswer {
                answer_content: "answer_content".to_string()
            },
        ];
        let question = UpdateQuestion {
            sku_code,
            product_code,
            question_code,
            question_content,
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
        let question_code = "question_code".to_string();
        let product_code = "product_code".to_string();
        let question_content = "".to_string();
        let rank = 1;
        let answer_list = vec![
            UpdateAnswer {
                answer_content: "answer_content".to_string()
            },
        ];
        let question = UpdateQuestion {
            sku_code,
            product_code,
            question_code,
            question_content,
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


    //模拟answers列表为空的情况
    #[tokio::test]
    async fn test_validate_answer_list_empty() {
        init_log().await;
        let sku_code = Some("sku_code".to_string());
        let question_code = "question_code".to_string();
        let product_code = "product_code".to_string();
        let question_content = "question_content".to_string();
        let rank = 1;
        let answer_list = vec![
        ];
        let question = UpdateQuestion {
            sku_code,
            product_code,
            question_code,
            question_content,
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
        let question_code = "question_code".to_string();
        let product_code = "product_code".to_string();
        let question_content = "question_content".to_string();
        let rank = 1;
        let answer_list = vec![
            UpdateAnswer {
                answer_content: "".to_string()
            },
        ];
        let question = UpdateQuestion {
            sku_code,
            product_code,
            question_code,
            question_content,
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

    

}
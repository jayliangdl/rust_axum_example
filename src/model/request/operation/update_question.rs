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
use crate::model::response::operation::update_question::UpdateQuestion as ResponseCreateQuestion;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct UpdateQuestion {
    pub sku_code: Option<String>,
    #[validate(length(min = 1, message = "product_code不能为空"))]
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[validate(length(min = 1, message = "question_content不能为空"))]
    #[serde(rename = "questionContent")]
    pub question_content: String,
    pub rank: i32,
    #[validate(custom = "validate_answer_list")]
    #[serde(rename = "answers")]
    pub answer_list: Vec<UpdateAnswer>,
    #[validate(length(min = 1, message = "question_code不能为空"))]
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
    #[validate(length(min = 1, message = "answer_content不能为空"))]
    #[serde(rename = "answerContent")]
    pub answer_content:String,
}

fn validate_answer_list(value: &Vec<UpdateAnswer>) -> Result<(), ValidationError> {
    if value.len() == 0 {
        return Err(ValidationError{code:"".into(),message:Some(Cow::from("answer_list列表不能为空".to_string())),params:HashMap::new()});
    }
    Ok(())
}

// #[cfg(test)]
// mod test {
//     use crate::model::request::operation::update_question::UpdateQuestion;
//     use crate::model::request::operation::update_question::UpdateAnswer;
//     use crate::models::ApiResponse;

//     //模拟product_code为空的情况
//     #[tokio::test]
//     async fn test_validate_product_code_empty() {
//         let sku_code = Some("sku_code".to_string());
//         let product_code = "".to_string();
//         let question_content = "question_content".to_string();
//         let rank = 1;
//         let answer_list = vec![
//             UpdateAnswer {
//                 answer_content: "answer_content".to_string(),
//             },
//         ];
//         let question_code = "question_code".to_string();
//         let question = UpdateQuestion {
//             sku_code,
//             product_code,
//             question_content,
//             rank,
//             answer_list,
//             question_code,
//         };
//         let result = question.custom_validate().await;
//         assert_eq!(result.is_err(), true);
//         if let Err(error) = result {
//             match error {
//                 ApiResponse::ERROR { msg, error_parameters, .. } => {
//                     assert_eq!(msgg, "入参错误");
//                     assert_eq!(
//                         error_parameters.unwrap().get("productCode")
//                             .unwrap().as_array()
//                             .unwrap().get(0)
//                             .unwrap().get("message").unwrap().as_str().unwrap(),
//                         "product_code不能为空"
//                     );
//                 }
//                 _ => {
//                     panic!("错误类型不匹配")
//                 }
//             }
//         }
//     }

//     //模拟question_content为空的情况
//     #[tokio::test]
//     async fn test_validate_question_content_empty() {
//         let sku_code = Some("sku_code".to_string());
//         let product_code = "product_code".to_string();
//         let question_content = "".to_string();

//         let rank = 1;
//         let answer_list = vec![
//             UpdateAnswer {
//                 answer_content: "answer_content".to_string(),
//             },
//         ];
//         let question_code = "question_code".to_string();
//         let question = UpdateQuestion {
//             sku_code,
//             product_code,
//             question_content,
//             rank,
//             answer_list,
//             question_code,
//         };
//         let result = question.custom_validate().await;
//         assert_eq!(result.is_err(), true);
//         if let Err(error) = result {
//             match error {
//                 ApiResponse::ERROR { msg, error_parameters, .. } => {
//                     assert_eq!(msg, "入参错误");
//                     assert_eq!(
//                         error_parameters.unwrap().get("questionContent")
//                             .unwrap().as_array()
//                             .unwrap().get(0)
//                             .unwrap().get("message").unwrap().as_str().unwrap(),
//                         "question_content不能为空"
//                     );
//                 }
//                 _ => {
//                     panic!("错误类型不匹配")
//                 }
//             }
//         }
//     }

//     //模拟answer_list为空的情况
//     #[tokio::test]
//     async fn test_validate_answer_list_empty() {
//         let sku_code = Some("sku_code".to_string());
//         let product_code = "product_code".to_string();
//         let question_content = "question_content".to_string();
//         let rank = 1;
//         let answer_list = vec![];
//         let question_code = "question_code".to_string();
//         let question = UpdateQuestion {
//             sku_code,
//             product_code,
//             question_content,
//             rank,
//             answer_list,
//             question_code
//         };
//         let result = question.custom_validate().await;
//         assert_eq!(result.is_err(), true);
//         if let Err(error) = result {
//             match error {
//                 ApiResponse::ERROR { msg, error_parameters, .. } => {
//                     assert_eq!(msg, "入参错误");
//                     assert_eq!(
//                         error_parameters.unwrap().get("answers")
//                             .unwrap().as_array()
//                             .unwrap().get(0)
//                             .unwrap().get("message").unwrap().as_str().unwrap(),
//                         "answer_list列表不能为空"
//                     );
//                 }
//                 _ => {
//                     panic!("错误类型不匹配")
//                 }
//             }
//         }
//     }

//     //模拟answer_content为空的情况
//     #[tokio::test]
//     async fn test_validate_answer_content_empty() {
//         let sku_code = Some("sku_code".to_string());
//         let product_code = "product_code".to_string();
//         let question_content = "question_content".to_string();
//         let rank = 1;
//         let answer_list = vec![
//             UpdateAnswer {
//                 answer_content: "".to_string(),
//             },
//         ];
//         let question_code = "question_code".to_string();
//         let question = UpdateQuestion {
//             sku_code,
//             product_code,
//             question_content,
//             rank,
//             answer_list,
//             question_code
//         };
//         let result = question.custom_validate().await;
//         assert_eq!(result.is_err(), true);
//         if let Err(error) = result {
//             match error {
//                 ApiResponse::ERROR { msg, error_parameters, .. } => {
//                     assert_eq!(msg, "入参错误");
//                     assert_eq!(
//                         error_parameters.unwrap().get("answerContent")
//                             .unwrap().as_array()
//                             .unwrap().get(0)
//                             .unwrap().get("message").unwrap().as_str().unwrap(),
//                         "answer_content不能为空"
//                     );
//                 }
//                 _ => {
//                     panic!("错误类型不匹配")
//                 }
//             }
//         }
//     }

//     //模拟question_code为空的情况
//     #[tokio::test]
//     async fn test_validate_question_code_empty() {
//         let sku_code = Some("sku_code".to_string());
//         let product_code = "product_code".to_string();
//         let question_content = "question_content".to_string();
//         let rank = 1;
//         let answer_list = vec![
//             UpdateAnswer {
//                 answer_content: "answer_content".to_string(),
//             },
//         ];
//         let question_code = "".to_string();
//         let question = UpdateQuestion {
//             sku_code,
//             product_code,
//             question_content,
//             rank,
//             answer_list,
//             question_code
//         };
//         let result = question.custom_validate().await;
//         assert_eq!(result.is_err(), true);
//         if let Err(error) = result {
//             match error {
//                 ApiResponse::ERROR { msg, error_parameters, .. } => {
//                     assert_eq!(msg, "入参错误");
//                     assert_eq!(
//                         error_parameters.unwrap().get("questionCode")
//                             .unwrap().as_array()
//                             .unwrap().get(0)
//                             .unwrap().get("message").unwrap().as_str().unwrap(),
//                         "question_code不能为空"
//                     );
//                 }
//                 _ => {
//                     panic!("错误类型不匹配")
//                 }
//             }
//         }
//     }

// }
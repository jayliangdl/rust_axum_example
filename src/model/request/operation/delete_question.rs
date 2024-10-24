use serde::{Serialize,Deserialize};
use validator::Validate;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct DeleteQuestion {
    #[validate(length(min = 1, message = "product_code不能为空"))]
    #[serde(rename = "questionCode")]
    pub question_code: String,
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
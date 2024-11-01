use serde::{Serialize,Deserialize};
use validator::Validate;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct FindQuestionListForTrad {

    #[serde(rename = "pageNum")]
    #[validate(range(min = 1, message = "页面参数不合法，pageNum应该是大于等于1的整数"))]
    pub current_pageno:i64,
    #[serde(rename = "pageSize")]
    #[validate(range(min = 1, message = "页面参数不合法，pageSize应该是大于等于1的整数"))]
    pub page_size:i64,

    #[serde(rename="idStr")]
    pub id_str: Option<String>,
    #[serde(rename = "productCode")]
    pub product_code: Option<String>,
    #[serde(rename = "questionContent")]
    pub question_content: Option<String>,
    #[serde(rename = "questionCode")]
    pub question_code: Option<String>,
    #[serde(rename = "creatorName")]
    pub create_name: Option<String>,
    #[serde(rename = "answerCode")]
    pub answer_code: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::NaiveDateTime>,
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::NaiveDateTime>,
    #[serde(rename = "answerContent")]
    pub answer_content:Option<String>
}

impl FindQuestionListForTrad {
    pub fn new() -> Self {
        FindQuestionListForTrad {
            current_pageno: 1,
            page_size: 10,
            id_str: None,
            product_code: None,
            question_content: None,
            question_code: None,
            create_name: None,
            answer_code: None,
            start_time: None,
            end_time: None,
            answer_content: None,
        }
    }
}

#[cfg(test)]
mod test {
    use validator::Validate;

    use crate::model::request::operation::find_question_list_for_trad::FindQuestionListForTrad;
    use crate::utils::error::BusinessError;
    use crate::utils::logging::init_log;

    //模拟current_pageno为负数、零和正数的情况
    #[tokio::test]
    async fn test_validate_current_pageno() {
        init_log().await;
        let possible_values = vec![0, -1];
        for value in possible_values{
            let request = FindQuestionListForTrad {
                current_pageno: value,
                page_size: 10,
                id_str: None,
                product_code: None,
                question_content: None,
                question_code: None,
                create_name: None,
                answer_code: None,
                start_time: None,
                end_time: None,
                answer_content: None,
            };
            let result = request.validate();
            assert_eq!(result.is_err(), true);
            if let Err(validation_errors) = result {
                let business_error = validation_errors.into();
                match business_error {
                    BusinessError::InvalidParameter ((_,errors_parameters)) => {
                        assert_eq!(errors_parameters.is_some(), true);
                        let error_parameters = errors_parameters.unwrap();
                        let error_parameters0 = error_parameters.get(0);
                        assert_eq!(error_parameters0.is_some(), true);
                        let error_parameters0 = error_parameters0.unwrap();
                        assert_eq!(error_parameters0.contains_key("field"), true);
                        assert_eq!(error_parameters0.contains_key("message"), true);
                        assert_eq!(error_parameters0.get("field").unwrap(),"pageNum");  
                        assert_eq!(error_parameters0.get("message").unwrap(),"页面参数不合法，pageNum应该是大于等于1的整数");  
                    }
                    _ => {
                        panic!("错误类型不匹配")
                    }
                }
            }
        }

        let request = FindQuestionListForTrad {
            current_pageno: 1,
            page_size: 10,
            id_str: None,
            product_code: None,
            question_content: None,
            question_code: None,
            create_name: None,
            answer_code: None,
            start_time: None,
            end_time: None,
            answer_content: None,
        };
        let result = request.validate();
        assert_eq!(result.is_err(), false);
    }

    //模拟page_size为负数、零和正数的情况
    #[tokio::test]
    async fn test_validate_page_size() {
        init_log().await;
        let possible_values = vec![0, -1];
        for value in possible_values{
            let request = FindQuestionListForTrad {
                current_pageno: 1,
                page_size: value,
                id_str: None,
                product_code: None,
                question_content: None,
                question_code: None,
                create_name: None,
                answer_code: None,
                start_time: None,
                end_time: None,
                answer_content: None,
            };
            let result = request.validate();
            assert_eq!(result.is_err(), true);
            if let Err(validation_errors) = result {
                let business_error = validation_errors.into();
                match business_error {
                    BusinessError::InvalidParameter ((_,errors_parameters)) => {
                        assert_eq!(errors_parameters.is_some(), true);
                        let error_parameters = errors_parameters.unwrap();
                        let error_parameters0 = error_parameters.get(0);
                        assert_eq!(error_parameters0.is_some(), true);
                        let error_parameters0 = error_parameters0.unwrap();
                        assert_eq!(error_parameters0.contains_key("field"), true);
                        assert_eq!(error_parameters0.contains_key("message"), true);
                        assert_eq!(error_parameters0.get("field").unwrap(),"pageSize");  
                        assert_eq!(error_parameters0.get("message").unwrap(),"页面参数不合法，pageSize应该是大于等于1的整数");  
                    }
                    _ => {
                        panic!("错误类型不匹配")
                    }
                }
            }
        }

        let request = FindQuestionListForTrad {
            current_pageno: 1,
            page_size: 10,
            id_str: None,
            product_code: None,
            question_content: None,
            question_code: None,
            create_name: None,
            answer_code: None,
            start_time: None,
            end_time: None,
            answer_content: None,
        };
        let result = request.validate();
        assert_eq!(result.is_err(), false);
    }

}

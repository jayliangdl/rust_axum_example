use serde::{Serialize,Deserialize};
use validator::Validate;

#[derive(Serialize,Deserialize, Debug, Validate, Clone)]
pub struct DeleteQuestion {
    #[validate(length(min = 1, message = "questionCode不能为空"))]
    #[serde(rename = "questionCode")]
    pub question_code: String,
}


#[cfg(test)]
mod test {
    use validator::Validate;

    use crate::model::request::operation::delete_question::DeleteQuestion as RequestDeleteQuestion;
    use crate::utils::error::BusinessError;
    use crate::utils::logging::init_log;

    //模拟product_code为空的情况
    #[tokio::test]
    async fn test_validate_product_code_empty() {
        init_log().await;
        let question_code = "".to_string();
        let question = RequestDeleteQuestion {
            question_code,
        };
        let result = question.validate();
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
                    assert_eq!(error_parameters0.get("field").unwrap(),"questionCode");  
                    assert_eq!(error_parameters0.get("message").unwrap(),"questionCode不能为空");  
                }
                _ => {
                    panic!("错误类型不匹配")
                }
            }
        }
    }

}
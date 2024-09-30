use std::{error::Error, fmt::Result as Result};
use actix_web::error;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ErrorCodes {
    Ok = 10000,
    ControllerErr = 20200,
}

impl std::fmt::Display for ErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            Self::Ok => write!(f, "Ok(正确)"),
            Self::ControllerErr => write!(f, "ControllerErr(控制器错误)"),
        }
    }
}


#[derive(Debug, Serialize)]
pub struct ControllerError {
    err_code: ErrorCodes,
    msg: String,
}

impl Error for ControllerError {}
impl std::fmt::Display for ControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "[{}] {}", self.err_code, self.msg)
    }
}

impl ControllerError {
    pub fn new(err:ErrorCodes) -> Self {
        let msg = format!("{}",err);
        Self {
            err_code: err,
            msg: msg
        }
    }
}

// 实现Active错误
impl error::ResponseError for ControllerError {}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::Result;

    #[tokio::test]
    async fn test1() -> Result<(), ControllerError> {
        let e = ControllerError{
            err_code: ErrorCodes::ControllerErr,
            msg: "错误".to_string()
        };

        // 使用 `matches!` 宏来匹配错误类型
        assert!(matches!(e, ControllerError { err_code: ErrorCodes::ControllerErr, .. }));

        Ok(())
    }

}

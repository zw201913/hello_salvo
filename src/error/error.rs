use salvo::{Depot, Request, Response, Writer, async_trait};
use salvo::http::StatusCode;
use salvo::prelude::Json;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct GlobalError {
    code: u16,
    // 提示信息
    msg: String,
    // 错误信息
    error: String,
}

impl GlobalError {
    pub fn new(code: u16, msg: &str, error: &str) -> GlobalError {
        GlobalError {
            code,
            msg: String::from(msg),
            error: String::from(error),
        }
    }

    pub fn bad_request(msg: &str, error: &str) -> GlobalError {
        GlobalError::new(StatusCode::BAD_REQUEST.as_u16(), msg, error)
    }

    pub fn write(self, res: &mut Response) {
        let status_code = StatusCode::from_u16(self.code);
        match status_code {
            Err(_) => {
                res.set_status_code(StatusCode::OK);
            }
            Ok(code) => {
                res.set_status_code(code);
            }
        }
        res.render(Json(self));
    }
}

#[async_trait]
impl Writer for GlobalError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        self.write(res);
    }
}
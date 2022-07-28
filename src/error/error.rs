use salvo::{Depot, Request, Response, Writer, async_trait};
use salvo::http::StatusCode;
use salvo::prelude::Json;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct GlobalError {
    // 提示信息
    msg: String,
    // 错误信息
    error: String,
}

impl GlobalError {
    pub fn new(msg: &str, error: &str) -> GlobalError {
        GlobalError {
            msg: String::from(msg),
            error: String::from(error),
        }
    }
}

#[async_trait]
impl Writer for GlobalError {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(self));
    }
}
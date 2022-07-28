use salvo::prelude::{Request, Response, Router, handler, Json, Extractible};
use salvo::http::header::{self, HeaderValue};
use tracing::instrument;
use crate::error::error::GlobalError;
use crate::service::account_service::AccountService;
use serde::{Serialize, Deserialize};

#[instrument]
pub fn init() -> Router {
    Router::new()
        .push(Router::with_path("/user_info/<id>").get(get_user_info))
        .push(Router::with_hoop(add_header).get(hello_world))
        .push(Router::with_path("/create_account").post(create_account))
}


#[handler]
async fn add_header(res: &mut Response) {
    res.headers_mut().insert(header::SERVER, HeaderValue::from_static("Salvo"));
}

#[handler]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[handler]
async fn get_user_info(req: &mut Request, res: &mut Response) {
    let option_id = req.param::<String>("id");
    match option_id {
        None => {
            panic!("id不合法");
        }
        Some(id) => {
            let account = AccountService::query_user_info_by_id(&id);
            res.render(Json(account));
        }
    }
}

#[handler]
async fn create_account(user_info: UserInfo, res: &mut Response) -> Result<String, GlobalError> {
    let account = &user_info.account;
    let password = &user_info.password;
    match AccountService::add_account(account, password) {
        Ok(x) => {
            println!("受影响的行数：{}",x);
            Ok(String::from("成功！"))
        },
        Err(e) => Err(e)
    }
}

#[derive(Debug, Serialize, Deserialize, Extractible)]
#[extract(
default_source(from = "body", format = "json")
)]
struct UserInfo {
    pub account: String,
    pub password: String,
}
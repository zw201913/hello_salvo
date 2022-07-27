use salvo::prelude::{Request, Response, Router, handler, Json};
use salvo::http::header::{self, HeaderValue};
use tracing::instrument;
use crate::service::account_service::UserService;

#[instrument]
pub fn init() -> Router {
    Router::new()
        .push(Router::with_path("/user_info/<id>").get(get_user_info))
        .push(Router::with_hoop(add_header).get(hello_world))
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
            let account = UserService::query_user_info_by_id(&id);
            res.render(Json(account));
        }
    }
}
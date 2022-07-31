use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use salvo::prelude::{Request, Response, Router, handler, Json, Extractible};
use salvo::http::header::{self, HeaderValue};
use tracing::{instrument, info};
use crate::error::error::GlobalError;
use crate::service::account_service::AccountService;
use serde::{Serialize, Deserialize};

#[instrument]
pub fn init() -> Router {
    Router::new()
        .push(Router::with_path("/user_info/<id>").get(get_user_info))
        .push(Router::with_hoop(add_header).get(hello_world))
        .push(Router::with_path("/create_account").post(create_account))
        .push(Router::with_path("/user/<id>").get(query_order_in_user).post(edit_order))
        .push(Router::with_path("/upload/<id>").post(upload))
}

#[handler]
async fn upload(file_name: FileName, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap();
    let file = req.file("file").await;
    if let Some(file) = file {
        // 目标路径
        let dest_file = format!("/Users/zouwei/Desktop/{}", &file.name().unwrap());
        match std::fs::copy(&file.path(), Path::new(&dest_file)) {
            Ok(_) => {
                res.render(format!("id:{} \nfile_name:{}  \n文件上传成功：{}", id, file_name.name, &dest_file));
            }
            Err(e) => {
                GlobalError::new(500, "file store failure", e.description()).write(res);
            }
        }
    } else {
        GlobalError::bad_request("file not found in request", "file not found in request").write(res);
    }
}


#[handler]
async fn user_info_by_id(req: &mut Request) -> String {
    let id = req.param::<String>("id").unwrap();
    id
}

// /user/<id>?oid={order_id}
#[handler]
async fn query_order_in_user(query: OrderQuery, res: &mut Response) {
    // 从路径上取参数
    let user_id = query.user_id;
    // 从?后面取参数
    let order_id = query.order_id;
    // 写出响应数据
    res.render(format!("user_id:{}\norder_id:{}", user_id, order_id));
}

#[derive(Debug, Serialize, Deserialize, Extractible)]
#[extract(
default_source(from = "param"),
default_source(from = "query")
)]
struct OrderQuery {
    #[extract(alias = "id", source(from = "param"))]
    user_id: String,
    #[extract(alias = "oid", source(from = "query"))]
    order_id: String,
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
            let msg = "id不合法";
            GlobalError::bad_request(msg, msg).write(res);
        }
        Some(id) => {
            let account = AccountService::query_user_info_by_id(&id);
            res.render(Json(account));
        }
    }
}

#[handler]
async fn create_account(req: &mut Request) -> Result<String, GlobalError> {
    let user_info: UserInfo = req.extract_body().await.unwrap();
    let account = &user_info.account;
    let password = &user_info.password;
    match AccountService::add_account(account, password) {
        Ok(x) => {
            info!("受影响的行数：{}", x);
            Ok(String::from("成功！"))
        }
        Err(e) => Err(e)
    }
}

#[handler]
async fn edit_order(order_save: OrderSave, req: &mut Request, res: &mut Response) {
    let content_type: String = req.header("Content-Type").unwrap();
    println!("{}", content_type);
    res.render(Json(order_save));
}

#[derive(Debug, Serialize, Deserialize, Extractible)]
struct OrderSave {
    // 从请求路径中取参数"id"的值
    #[extract(alias = "id", source(from = "param"))]
    user_id: String,
    // 从?后面取参数"oid"的值
    #[extract(alias = "oid", source(from = "query"))]
    order_id: String,
    #[extract(alias = "Cookie", source(from = "header"))]
    cookie: String,
    // 从请求体中取数据
    #[extract(source(from = "body", format = "json"))]
    title: String,
    #[extract(source(from = "body", format = "json"))]
    price: f64,
}


#[derive(Debug, Serialize, Deserialize, Extractible)]
#[extract(
default_source(from = "body", format = "json")
)]
struct UserInfo {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Extractible)]
struct FileName {
    #[extract(source(from = "body"))]
    name: String,
}
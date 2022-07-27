use salvo::Router;
use tracing::{instrument, info};

mod user_controller;
mod vo;

#[instrument]
pub fn init() -> Router {
    info!("收集所有的请求路由配置---------------开始---------------");
    let router = Router::new()
        .push(user_controller::init());
    info!(router=?router);
    info!("收集所有的请求路由配置---------------结束---------------");
    router
}
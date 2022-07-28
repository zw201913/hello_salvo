mod dao;
mod controller;
mod logs;
mod service;
mod error;

use salvo::prelude::{Server, TcpListener};
use tracing::{span, info, Level};

// 启动服务器
fn start_server(ip: &str, port: usize) -> Server<TcpListener> {
    Server::new(TcpListener::bind(&format!("{}:{}", ip, port)))
}

#[tokio::main]
async fn main() {
    // 初始化日志
    logs::init();
    let span = span!(Level::WARN, "main");
    let _enter = span.enter();
    info!("main function start");
    // 初始化数据库连接池
    dao::init();
    // 初始化service
    service::init();
    // 初始化请求路由
    let router = controller::init();
    // 启动服务
    start_server("127.0.0.1", 7878).serve(router).await;
}

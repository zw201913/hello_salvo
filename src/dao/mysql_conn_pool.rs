use mysql::{Pool, PooledConn};
use once_cell::sync::OnceCell;
use tracing::{instrument, info};

// 创建一个全局的DB_POOL，可以一直使用，启动的时候初始化即可
static DB_POOL: OnceCell<Pool> = OnceCell::new();

// 初始化数据库链接池
#[instrument]
pub fn init_mysql_pool(db_url: &str) {
    info!("初始化数据库线程池--------开始-------");
    DB_POOL.set(mysql::Pool::new(&db_url).expect(&format!("Error connecting to {}", &db_url)))
        .unwrap_or_else(|_| { info!("try insert pool cell failure!") });
    info!("初始化数据库线程池--------结束-------");
}

// 从链接链接池里面获取链接
#[instrument]
pub fn get_connect() -> PooledConn {
    info!("从链接池获取数据库链接----------开始----------");
    let conn = DB_POOL.get().expect("Error get pool from OneCell<Pool>").get_conn().expect("Error get_connect from db pool");
    info!("从链接池获取数据库链接----------结束----------");
    conn
}
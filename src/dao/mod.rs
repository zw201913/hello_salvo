mod mysql_conn_pool;
pub mod account_mapper;
pub mod po;

const DB_URL: &str = "mysql://数据库用户名:数据库密码@数据库ip:数据库端口/数据库名称";

pub fn init() {
    // 初始化链接池
    mysql_conn_pool::init_mysql_pool(DB_URL);
}


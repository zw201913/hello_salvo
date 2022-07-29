use mysql::prelude::{BinQuery, Queryable, WithParams};
use mysql::params;
use crate::dao::mysql_conn_pool::get_connect;
use crate::dao::po::account::Account;
use tracing::error;
use std::error::Error;
use crate::error::error::GlobalError;

pub struct AccountMapper;

impl AccountMapper {
    pub fn get_by_id(id: &str) -> Option<Account> {
        // 获取数据库链接
        let mut conn = get_connect();
        // 根据id查询账号信息
        let query_result = conn.exec_first("select id,account,password,enabled,create_time,modify_time from account where id=:id", params!("id"=>id))
            .map(|row| {
                row.map(|(id, account, password, enabled, create_time, modify_time)| Account { id, account, password, enabled, create_time, modify_time })
            });
        // 判断是否查询到数据
        match query_result {
            Ok(result) => {
                result
            }
            Err(_) => {
                None
            }
        }
    }

    pub fn insert(account: &str, password: &str) -> Result<u64, GlobalError> {
        // 获取数据库链接
        let mut conn = get_connect();
        // 执行插入语句，目前id写死，后续会修改
        let x = match "insert into account (id,account,password,enabled,create_time,modify_time) values (?,?,?,1,now(),now())"
            .with(("123456", account, password))
            .run(&mut conn) {
            // 返回受影响的数据行数
            Ok(res) => {
                Ok(res.affected_rows())
            }
            Err(e) => {
                // error!(e);
                Err(GlobalError::new(200, "创建账号失败", e.to_string().as_str()))
            }
        };
        x
    }
}

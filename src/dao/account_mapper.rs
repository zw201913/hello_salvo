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
        let mut conn = get_connect();
        let query_result = conn.exec_first("select id,account,password,enable,create_time,modify_time from account where id=:id", params!("id"=>id))
            .map(|row| {
                row.map(|(id, account, password, enable, create_time, modify_time)| Account { id, account, password, enable, create_time, modify_time })
            });
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
        let mut conn = get_connect();
        let x = match "insert into account (id,account,password,enable,create_time,modify_time) values (?,?,?,1,now(),now())"
            .with(("12345", account, password))
            .run(&mut conn) {
            Ok(res) => {
                Ok(res.affected_rows())
            }
            Err(e) => {
                // error!(e);
                Err(GlobalError::new("创建账号失败", e.to_string().as_str()))
            }
        };
        x
    }
}

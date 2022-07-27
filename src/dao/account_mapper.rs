use mysql::prelude::{Queryable};
use mysql::params;
use crate::dao::mysql_conn_pool::get_connect;
use crate::dao::po::account::Account;

pub struct AccountMapper;

impl AccountMapper {
    pub fn get_account_by_id(id: &str) -> Option<Account> {
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
}

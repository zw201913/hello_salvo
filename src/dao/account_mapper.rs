use mysql::prelude::{BinQuery, Queryable, WithParams};
use mysql::params;
use crate::dao::mysql_conn_pool::get_connect;
use crate::dao::po::account::Account;
use crate::error::error::GlobalError;
use crate::utils;

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
        // 生成主键id
        let id = utils::generate_id()?;
        // 执行插入语句，目前id写死，后续会修改
        let x = match "insert into account (id,account,password,enabled,create_time,modify_time) values (?,?,?,1,now(),now())"
            .with((id, account, password))
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

    pub fn delete_by_id(id: &str) -> Result<u64, GlobalError> {
        let mut conn = get_connect();
        let x = match "DELETE FROM account WHERE id=?"
            .with((id, ))
            .run(&mut conn) {
            Ok(res) => {
                Ok(res.affected_rows())
            }
            Err(e) => {
                Err(GlobalError::new(200, "删除用户失败", e.to_string().as_str()))
            }
        };
        x
    }

    pub fn update(account: Account) -> Result<u64, GlobalError> {
        let mut conn = get_connect();
        let x = match "UPDATE account SET account=?, password=?,enabled=?, modify_time=now() where id=?".with((&account.account, &account.password, &account.enabled, &account.id)).run(&mut conn) {
            Ok(res) => {
                Ok(res.affected_rows())
            }
            Err(e) => {
                Err(GlobalError::new(200, "用户信息更新失败", e.to_string().as_str()))
            }
        };
        x
    }
}


#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;
    use crate::dao::account_mapper::AccountMapper;
    use crate::dao::po::account::Account;
    use crate::dao;

    #[test]
    pub fn get_by_id_test() {
        dao::init();
        let res = AccountMapper::get_by_id("1");
        assert_eq!(res, Some(Account {
            id: String::from("1"),
            account: String::from("zouwei"),
            password: String::from("123456"),
            enabled: 1,
            create_time: NaiveDateTime::parse_from_str("2022-07-28 17:08:19", "yyyy-MM-dd HH:mm:ss").unwrap(),
            modify_time: NaiveDateTime::parse_from_str("2022-07-28 17:08:19", "yyyy-MM-dd HH:mm:ss").unwrap(),
        }));
    }

    #[test]
    pub fn insert_test() {
        dao::init();
        let res = AccountMapper::insert("zouwei", "098765");
        assert_eq!(res, Ok(1));
    }


    #[test]
    pub fn delete_by_id_test() {
        dao::init();
        let res = AccountMapper::delete_by_id("1");
        assert_eq!(res, Ok(1));
    }

    #[test]
    pub fn update_test() {
        dao::init();
        let data_time = chrono::offset::Utc::now();
        let res = AccountMapper::update(Account {
            id: String::from("2"),
            account: String::from("zouwei"),
            password: String::from("123456"),
            enabled: 1,
            create_time: NaiveDateTime::from_timestamp(data_time.timestamp(), 0),
            modify_time: NaiveDateTime::from_timestamp(data_time.timestamp(), 0),
        });
        assert_eq!(res, Ok(1));
    }
}
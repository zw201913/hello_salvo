use crate::dao::po::account::Account;
use crate::dao::account_mapper::AccountMapper;

pub struct UserService;

impl UserService {
    pub fn query_user_info_by_id(id: &str) -> Account {
        if id.len() <= 0 {
            panic!("id不合法");
        }
        let query_result = AccountMapper::get_account_by_id(id);
        match query_result {
            None => {
                panic!("没有查到任何数据");
            }
            Some(account) => {
                account
            }
        }
    }
}
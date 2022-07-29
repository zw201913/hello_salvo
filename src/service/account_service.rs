use crate::dao::po::account::Account;
use crate::dao::account_mapper::AccountMapper;
use crate::error::error::GlobalError;

pub struct AccountService;

impl AccountService {
    pub fn query_user_info_by_id(id: &str) -> Account {
        if id.len() <= 0 {
            panic!("id不合法");
        }
        let query_result = AccountMapper::get_by_id(id);
        match query_result {
            None => {
                panic!("没有查到任何数据");
            }
            Some(account) => {
                account
            }
        }
    }

    pub fn add_account(account: &str, password: &str) -> Result<u64, GlobalError> {
        AccountMapper::insert(account, password)
    }
}
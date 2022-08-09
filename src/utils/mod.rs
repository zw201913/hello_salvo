use once_cell::sync::OnceCell;
use crate::utils::id_generator::SnowflakeIdWorker;
use lazy_static::lazy_static;
use constant::GLOBAL_CONFIG;
use crate::error::error::GlobalError;

pub mod id_generator;
pub mod constant;


lazy_static! {
    pub static ref SNOW_FLAKE_ID_WORKER:SnowflakeIdWorker={
        let work_id = GLOBAL_CONFIG.snow_flake_id_worker.work_id as u128;
        let data_center_id=GLOBAL_CONFIG.snow_flake_id_worker.data_center_id as u128;
        SnowflakeIdWorker::new(work_id,data_center_id).unwrap()
    };


}

// 生成主键id
pub fn generate_id() -> Result<u128, GlobalError> {
    match SNOW_FLAKE_ID_WORKER.next_id() {
        Ok(id) => {
            Ok(id)
        }
        Err(e) => {
            return Err(GlobalError::new(200, "主键id生成失败", e.to_string().as_str()));
        }
    }
}

pub fn init() {
    //
}
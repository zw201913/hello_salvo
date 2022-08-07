use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, Error};

// 开始时间戳（2022-08-01）
const TWEPOCH: u128 = 1659283200000;
// 机器id所占的位数
const WORKER_ID_BITS: u128 = 5;
// 数据节点所占的位数
const DATA_CENTER_ID_BITS: u128 = 5;
// 支持最大的机器ID，最大是31
const MAX_WORKER_ID: u128 = (-1 ^ (-1 << WORKER_ID_BITS)) as u128;
// 支持的最大数据节点ID，结果是31
const MAX_DATA_CENTER_ID: u128 = (-1 ^ (-1 << DATA_CENTER_ID_BITS)) as u128;
// 序列号所占的位数
const SEQUENCE_BITS: u128 = 12;
// 工作节点标识ID向左移12位
const WORKER_ID_SHIFT: u128 = SEQUENCE_BITS;
// 数据节点标识ID向左移动17位（12位序列号+5位工作节点）
const DATA_CENTER_ID_SHIFT: u128 = SEQUENCE_BITS + WORKER_ID_BITS;
// 时间戳向左移动22位（12位序列号+5位工作节点+5位数据节点）
const TIMESTAMP_LEFT_SHIFT: u128 = SEQUENCE_BITS + WORKER_ID_BITS + DATA_CENTER_ID_BITS;
// 生成的序列掩码，这里是4095
const SEQUENCE_MASK: u128 = (-1 ^ (-1 << SEQUENCE_BITS)) as u128;


#[derive(Clone)]
pub struct SnowflakeIdWorker(Arc<Mutex<SnowflakeIdWorkerInner>>);

impl SnowflakeIdWorker {
    pub fn new(worker_id: u128, data_center_id: u128) -> Result<SnowflakeIdWorker> {
        Ok(
            Self(Arc::new(Mutex::new(SnowflakeIdWorkerInner::new(worker_id, data_center_id)?)))
        )
    }

    pub fn next_id(&self) -> Result<u128> {
        let mut inner = self.0.lock().map_err(|e| Error::msg(e.to_string()))?;
        inner.next_id()
    }
}

// 这是一个内部结构体，只在这个mod里面使用
struct SnowflakeIdWorkerInner {
    // 工作节点ID
    worker_id: u128,
    // 数据节点ID
    data_center_id: u128,
    // 序列号
    sequence: u128,
    // 上一次时间戳
    last_timestamp: u128,
}

impl SnowflakeIdWorkerInner {
    fn new(worker_id: u128, data_center_id: u128) -> Result<SnowflakeIdWorkerInner> {
        // 校验worker_id合法性
        if worker_id > MAX_WORKER_ID {
            return Err(Error::msg(format!("workerId:{} must be less than {}", worker_id, MAX_WORKER_ID)));
        }
        // 校验data_center_id合法性
        if data_center_id > MAX_DATA_CENTER_ID {
            return Err(Error::msg(format!("datacenterId:{} must be less than {}", data_center_id, MAX_DATA_CENTER_ID)));
        }
        // 创建SnowflakeIdWorkerInner对象
        Ok(SnowflakeIdWorkerInner {
            worker_id,
            data_center_id,
            sequence: 0,
            last_timestamp: 0,
        })
    }

    // 获取下一个id
    fn next_id(&mut self) -> Result<u128> {
        // 获取当前时间戳
        let mut timestamp = Self::get_time()?;
        // 如果当前时间戳小于上一次的时间戳，那么跑异常
        if timestamp < self.last_timestamp {
            return Err(Error::msg(format!("Clock moved backwards.  Refusing to generate id for {} milliseconds", self.last_timestamp - timestamp)));
        }
        // 如果当前时间戳等于上一次的时间戳，那么计算出序列号目前是第几位
        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & SEQUENCE_MASK;
            // 如果计算出来的序列号等于0，那么重新获取当前时间戳
            if self.sequence == 0 {
                timestamp = Self::til_next_mills(self.last_timestamp)?;
            }
        } else {
            // 如果当前时间戳大于上一次的时间戳，序列号置为0。因为又开始了新的毫秒，所以序列号要从0开始。
            self.sequence = 0;
        }
        // 把当前时间戳赋值给last_timestamp，以便下一次计算next_id
        self.last_timestamp = timestamp;
        // 把上面计算得到的对应数值按要求移位拼接起来
        Ok(((timestamp - TWEPOCH) << TIMESTAMP_LEFT_SHIFT)
            | (self.data_center_id << DATA_CENTER_ID_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | self.sequence)
    }
    // 计算一个大于上一次时间戳的时间戳
    fn til_next_mills(last_timestamp: u128) -> Result<u128> {
        // 获取当前时间戳
        let mut timestamp = Self::get_time()?;
        // 如果当前时间戳一直小于上次时间戳，那么一直循环获取，直至当前时间戳大于上次获取的时间戳
        while timestamp <= last_timestamp {
            timestamp = Self::get_time()?;
        }
        // 返回满足要求的时间戳
        Ok(timestamp)
    }

    // 获取当前时间戳
    fn get_time() -> Result<u128> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(s) => {
                Ok(s.as_millis())
            }
            Err(_) => {
                Err(Error::msg("get_time error!"))
            }
        }
    }
}
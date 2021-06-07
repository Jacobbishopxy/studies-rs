use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::Mutex;

/// serde的宏赋予结构体序列化与反序列化的功能
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Simulation {
    pub id: u64,
    pub name: String,
}

/// 实现结构体自定义的相等逻辑
impl PartialEq for Simulation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// 因为 Eq 是标记特性，需要重新标记一下
impl Eq for Simulation {}

/// 实现结构体自定义的哈希逻辑
/// 用于把 Simulation 放入 HashSet 内所需要的 Hash 特性
/// 如果所比较的值相等时，它们的 hash 同样也需要是相等的
/// 这里只需要关心 `id` 是否相等
impl Hash for Simulation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// 使用 Arc 智能指针目的在于为 HashSet 的引用提供线程安全。
/// 使用 `Mutex` 允许并行读取（提供读和写的锁）。
pub type Db = Arc<Mutex<HashSet<Simulation>>>;

pub fn new_db() -> Db {
    Arc::new(Mutex::new(HashSet::new()))
}

/// 用于根据 `id` 查找出 HashSet 内的 Simulation
pub fn get_simulation<'a>(sims: &'a HashSet<Simulation>, id: u64) -> Option<&'a Simulation> {
    sims.get(&Simulation {
        id,
        name: String::new(),
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewName {
    pub name: String,
}

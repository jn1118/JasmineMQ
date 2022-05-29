use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::config::{ADDRS, BROKER_COUNT};

pub fn find_leader(topic: &str) -> String {
    let mut s = DefaultHasher::new();
    topic.hash(&mut s);
    let hash = s.finish() as usize;
    let idx = hash % (BROKER_COUNT) as usize;
    ADDRS[idx].to_string()
}

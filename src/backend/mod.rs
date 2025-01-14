use dashmap::{DashMap, DashSet};
use std::{collections::BTreeMap, ops::Deref, sync::Arc};

use crate::{BulkString, RespArray, RespFrame};

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug, Clone)]
pub struct BackendInner {
    map: DashMap<String, RespFrame>,
    hmap: DashMap<String, DashMap<String, RespFrame>>,
    hset: DashMap<String, DashSet<String>>,
}

impl BackendInner {
    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|field_map| field_map.get(field).map(|v| v.value().clone()))
    }

    pub fn hmget(&self, key: &str, fields: &[String]) -> Option<Vec<Option<RespFrame>>> {
        self.hmap.get(key).map(|field_map| {
            fields
                .iter()
                .map(|field| field_map.get(field).map(|v| v.value().clone()))
                .collect()
        })
    }

    pub fn hmset(&self, key: String, fields: Vec<String>, values: Vec<RespFrame>) -> RespFrame {
        let hmap = self.hmap.entry(key).or_default();
        let success_count = fields
            .into_iter()
            .zip(values)
            .map(|(field, value)| hmap.insert(field, value))
            .filter(Option::is_none)
            .count();
        RespFrame::Integer(success_count as i64)
    }

    pub fn hgetall(&self, key: &str) -> Option<RespFrame> {
        self.hmap.get(key).map(|field_map| {
            let mut map = BTreeMap::new();
            field_map.iter().for_each(|entry| {
                map.insert(entry.key().clone(), entry.value().clone());
            });
            let mut vec = Vec::with_capacity(map.len() * 2);
            map.into_iter().for_each(|(key, value)| {
                vec.push(BulkString::new(Some(key)).into());
                vec.push(value);
            });
            RespFrame::Array(RespArray::new(Some(vec)))
        })
    }

    pub fn sadd(&self, key: String, fields: Vec<String>) -> RespFrame {
        let field_set = self.hset.entry(key).or_default();
        let success_count = fields
            .into_iter()
            .map(|field| field_set.insert(field))
            .filter(|b| *b)
            .count();
        RespFrame::Integer(success_count as i64)
    }

    pub fn smembers(&self, key: &str) -> Option<DashSet<String>> {
        self.hset.get(key).map(|field_set| field_set.clone())
    }

    pub fn sismember(&self, key: &str, field: &str) -> RespFrame {
        self.hset
            .get(key)
            .map(|field_map| {
                if field_map.contains(field) {
                    RespFrame::Integer(1)
                } else {
                    RespFrame::Integer(0)
                }
            })
            .unwrap_or(RespFrame::Integer(0))
    }
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Backend {
    pub fn new() -> Self {
        Backend(Arc::new(BackendInner::new()))
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}

impl BackendInner {
    fn new() -> Self {
        BackendInner {
            map: DashMap::new(),
            hmap: DashMap::new(),
            hset: DashMap::new(),
        }
    }
}

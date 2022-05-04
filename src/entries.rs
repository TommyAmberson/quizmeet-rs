use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Entry {
    fn get_name(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamEntry {
    pub name: String,
    pub quiz: String,
    pub place: f64,
    pub score: i32,
    pub points: i32,
    pub errors: i32,
}

impl Entry for TeamEntry {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizzerEntry {
    pub name: String,
    pub team: String,
    pub quiz: String,
    pub points: i32,
    pub errors: i32,
    pub jumps: i32,
    pub refer: i32,
    pub ftv: i32,
    pub int: i32,
    pub ma: i32,
    pub q: i32,
    pub sit: i32,
}

impl Entry for QuizzerEntry {
    fn get_name(&self) -> &str {
        &self.name
    }
}

pub fn group_by_name<T>(entries: Vec<T>) -> HashMap<String, Vec<T>>
where
    T: Entry,
{
    let mut map: HashMap<String, Vec<T>> = HashMap::new();
    for entry in entries {
        let name = entry.get_name();
        if !map.contains_key(name) {
            map.insert(String::from(name), Vec::new());
        }
        map.get_mut(name).unwrap().push(entry);
        // match map.get_mut(name) {
        //     Some(v) => v.push(entry),
        //     _ => (),
        // }
    }
    map
}

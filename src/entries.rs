use core::cmp::Ordering;
use serde::{Deserialize, Serialize};
use simple_error::bail;
use std::collections::HashMap;

pub trait Entry {
    fn get_name(&self) -> &str;
    fn add_to<'a>(&self, accum: &'a mut Self) -> Result<&'a mut Self, Box<dyn std::error::Error>>;
    fn empty() -> Self;
    fn avg(&self) -> f32;
    fn calc(&self) -> f32;
    fn cmp(&self, other: &Self) -> Ordering;
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
    fn add_to<'a>(&self, accum: &'a mut Self) -> Result<&'a mut Self, Box<dyn std::error::Error>> {
        if accum.name == "" {
            accum.name = self.name.clone();
        }
        if self.name != accum.name {
            bail!("needs same name")
        }

        if accum.quiz.len() > 0 {
            accum.quiz += ",";
        }
        accum.quiz += &self.quiz;

        accum.place += self.place;
        accum.score += self.score;
        accum.points += self.points;
        accum.errors += self.errors;
        Ok(accum)
    }
    fn empty() -> Self {
        Self {
            name: String::from(""),
            quiz: String::from(""),
            place: 0.,
            score: 0,
            points: 0,
            errors: 0,
        }
    }
    fn avg(&self) -> f32 {
        (self.points as f32) / ((self.quiz.matches(",").count() + 1) as f32)
    }
    fn calc(&self) -> f32 {
        ((self.points as f32) + (self.score as f32) / 10000.)
            / ((self.quiz.matches(",").count() + 1) as f32)
    }
    fn cmp(&self, other: &Self) -> Ordering {
        let calc1: i64 = ((self.points as i64) << 32) + (self.score as i64);
        let calc2: i64 = ((other.points as i64) << 32) + (other.score as i64);
        calc1.cmp(&calc2)
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
    fn add_to<'a>(&self, accum: &'a mut Self) -> Result<&'a mut Self, Box<dyn std::error::Error>> {
        if accum.name == "" {
            accum.name = self.name.clone();
        }
        if self.name != accum.name {
            bail!("needs same name")
        }
        if accum.team == "" {
            accum.team = self.team.clone();
        }
        if self.team != accum.team {
            bail!("needs same team")
        }

        if accum.quiz.len() > 0 {
            accum.quiz += ",";
        }
        accum.quiz += &self.quiz;

        accum.points += self.points;
        accum.errors += self.errors;
        accum.jumps += self.jumps;
        accum.refer += self.refer;
        accum.ftv += self.ftv;
        accum.int += self.int;
        accum.ma += self.ma;
        accum.q += self.q;
        accum.sit += self.sit;
        Ok(accum)
    }
    fn empty() -> Self {
        Self {
            name: String::from(""),
            team: String::from(""),
            quiz: String::from(""),
            points: 0,
            errors: 0,
            jumps: 0,
            refer: 0,
            ftv: 0,
            int: 0,
            ma: 0,
            q: 0,
            sit: 0,
        }
    }
    fn avg(&self) -> f32 {
        self.points as f32 / ((self.quiz.matches(",").count() + 1) as f32)
    }
    fn calc(&self) -> f32 {
        self.avg()
            + if self.errors > 0 {
                1. / (self.errors as f32 * 1000.)
            } else {
                0.001
            }
    }
    fn cmp(&self, other: &Self) -> Ordering {
        self.calc()
            .partial_cmp(&other.calc())
            .unwrap_or(Ordering::Equal)
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

pub fn sum<T>(entries: Vec<T>) -> Result<T, Box<dyn std::error::Error>>
where
    T: Entry,
{
    let mut accum: T = T::empty();
    for entry in entries {
        entry.add_to(&mut accum)?;
    }
    Ok(accum)
}

pub fn group_and_sum<T>(entries: Vec<T>) -> Result<HashMap<String, T>, Box<dyn std::error::Error>>
where
    T: Entry,
{
    group_by_name(entries)
        .into_iter()
        .map(|(k, v)| {
            let v = sum(v)?;
            Ok((k, v))
        })
        .collect()
}

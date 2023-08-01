use std::cmp::Ordering;

use anyhow::Result;

pub mod quizzer;
pub mod record;
pub mod team;

pub trait Stats<T: std::fmt::Debug>: From<T> + std::fmt::Debug {
    fn update(&mut self, entry: T) -> Result<()>;
    fn avg(&self) -> f32;
    fn tie_breaker(&self) -> f32;
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.avg() + self.tie_breaker() / 10000.;
        let b = other.avg() + other.tie_breaker() / 10000.;
        a.partial_cmp(&b)
    }
}

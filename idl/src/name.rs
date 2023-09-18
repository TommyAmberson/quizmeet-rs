use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct QuizzerName(pub String);
impl Display for QuizzerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<S: Into<String>> From<S> for QuizzerName {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}
impl AsRef<str> for QuizzerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct TeamName(pub String);
impl Display for TeamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<S: Into<String>> From<S> for TeamName {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}
impl AsRef<str> for TeamName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct QuizName(pub String);
impl Display for QuizName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<S: Into<String>> From<S> for QuizName {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}
impl AsRef<str> for QuizName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

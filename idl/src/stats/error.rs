use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatsError {
    #[error("Name must be the same for stats: '{stats}' and entry: '{entry:?}'")]
    BadName { stats: String, entry: String },
    #[error("Stats: '{stats}' already contains entry: '{entry:?}'")]
    DuplicateEntry { stats: String, entry: String },
}

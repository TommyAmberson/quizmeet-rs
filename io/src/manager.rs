use std::collections::HashMap;
use std::path::PathBuf;

use quizmeet_rs_idl::stats::record::StatsRecord;

use crate::quiz_file::QuizFile;

#[derive(Debug, PartialEq)]
pub struct Manager {
    pub root: PathBuf,
    pub files: HashMap<PathBuf, QuizFile>,
    pub stats: StatsRecord,
}

impl Manager {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        let root = root.into();
        Self {
            root,
            files: HashMap::new(),
            stats: StatsRecord::new(),
        }
    }
    pub async fn open_glob(&mut self, g: &str) -> anyhow::Result<()> {
        for path in glob::glob(g)? {
            self.open_path(path?).await?;
        }
        Ok(())
    }
    pub async fn open_path<P: Into<PathBuf>>(&mut self, path: P) -> anyhow::Result<()> {
        let path = path.into();
        let q = QuizFile::new(&path).await?;
        self.files.insert(path, q);
        Ok(())
    }
}

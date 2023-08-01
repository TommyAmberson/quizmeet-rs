use std::future::Future;
use std::path::PathBuf;
use std::{fs, io};

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use tokio::task::spawn_blocking;

#[derive(Debug, PartialEq, Eq)]
pub struct QuizFile {
    pub path: PathBuf,
    pub hash: [u8; 32],
}

impl QuizFile {
    pub async fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        let hash = get_hash(path.clone()).await?;
        Ok(Self { path, hash })
    }
    pub async fn if_changed<F: Future<Output = Result<()>>>(&mut self, func: F) -> Result<bool> {
        let new_hash = get_hash(self.path.clone()).await?;
        if new_hash == self.hash {
            Ok(false)
        } else {
            func.await?;
            self.hash = new_hash;
            Ok(true)
        }
    }
}

pub async fn get_hash(path: PathBuf) -> Result<[u8; 32]> {
    spawn_blocking(move || {
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let _n_bytes = io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();
        Ok(hash.into())
    })
    .await
    .with_context(|| "Failed to get hash")?
}

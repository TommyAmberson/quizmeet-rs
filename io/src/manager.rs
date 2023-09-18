use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use quizmeet_rs_idl::quiz::Quiz;
use quizmeet_rs_idl::stats::team::TeamStats;
use quizmeet_rs_idl::stats::Stats;
use redis::aio::Connection;
use redis::{AsyncCommands, Client, RedisResult};

use crate::parse_ods::read_from_file;
use crate::quiz_file::QuizFile;

pub struct Manager {
    pub root: PathBuf,
    pub files: HashMap<PathBuf, QuizFile>,
    pub connection: Connection,
}

impl Manager {
    pub fn new<P: Into<PathBuf>>(root: P, connection: Connection) -> Self {
        let root = root.into();
        Self {
            root,
            files: HashMap::new(),
            connection,
        }
    }
    pub async fn open_glob(&mut self, g: &str) -> Result<()> {
        for path in glob::glob(g)? {
            self.open_path(path?).await?;
        }
        Ok(())
    }
    pub async fn open_path<P: Into<PathBuf>>(&mut self, path: P) -> Result<()> {
        let path: PathBuf = path.into();
        let q = QuizFile::new(&path).await?;
        self.files.insert(path.clone(), q);
        let quiz = tokio::task::spawn_blocking(move || read_from_file(&path)).await??;
        self.update(quiz).await?;
        Ok(())
    }
    pub async fn update(&mut self, quiz: Quiz) -> Result<()> {
        for team_entry in quiz.team_entries {
            let name: String = team_entry.name.to_string();
            let mut team_stats: TeamStats = self.connection.get(&name).await?;
            // .or_insert_with(|| TeamStats::new(team_entry.name.clone()));

            let updated = team_stats.update(team_entry)?;
            if updated {
                log::trace!(
                    "Team: '{}' now has an average of '{}'",
                    team_stats.name,
                    team_stats.avg()
                );
                self.connection.set(&name, team_stats).await?;
            }

            // match self.teams.get_mut(&team_entry.name) {
            //     Some(team_stats) => {
            //         team_stats.update(team_entry)?;
            //         log::trace!(
            //             "Team: '{}' now has an average of '{}'",
            //             team_stats.name,
            //             team_stats.avg()
            //         );
            //     }
            //     None => {
            //         let team_stats: TeamStats = team_entry.into();
            //         log::trace!(
            //             "Team: '{}' created with initial average of '{}'",
            //             team_stats.name,
            //             team_stats.avg()
            //         );
            //         self.teams.insert(team_stats.name.clone(), team_stats);
            //     }
            // }
        }
        for quizzer_entry in quiz.quizzer_entries {
            // match self.quizzers.get_mut(&quizzer_entry.name) {
            //     Some(quizzer_stats) => {
            //         quizzer_stats.update(quizzer_entry)?;
            //         log::trace!(
            //             "Quizzer: '{}' now has an average of '{}'",
            //             quizzer_stats.name,
            //             quizzer_stats.avg()
            //         );
            //     }
            //     None => {
            //         let quizzer_stats: QuizzerStats = quizzer_entry.into();
            //         log::trace!(
            //             "Quizzer: '{}' created with initial average of '{}'",
            //             quizzer_stats.name,
            //             quizzer_stats.avg()
            //         );
            //         self.quizzers
            //             .insert(quizzer_stats.name.clone(), quizzer_stats);
            //     }
            // }
        }
        Ok(())
    }
}

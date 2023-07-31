use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::quiz::Quiz;
use crate::stats::error::StatsError;
use crate::stats::quizzer::QuizzerStats;
use crate::stats::team::TeamStats;
use crate::stats::Stats;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StatsRecord {
    pub teams: HashMap<String, TeamStats>,
    pub quizzers: HashMap<String, QuizzerStats>,
}

impl StatsRecord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, quiz: Quiz) -> Result<(), StatsError> {
        for team_entry in quiz.team_entries {
            match self.teams.get_mut(&team_entry.name) {
                Some(team_stats) => {
                    team_stats.add(team_entry)?;
                    log::trace!(
                        "Team: '{}' now has an average of '{}'",
                        team_stats.name,
                        team_stats.avg()
                    );
                }
                None => {
                    let team_stats: TeamStats = team_entry.into();
                    log::trace!(
                        "Team: '{}' created with initial average of '{}'",
                        team_stats.name,
                        team_stats.avg()
                    );
                    self.teams.insert(team_stats.name.clone(), team_stats);
                }
            }
        }
        for quizzer_entry in quiz.quizzer_entries {
            match self.quizzers.get_mut(&quizzer_entry.name) {
                Some(quizzer_stats) => {
                    quizzer_stats.add(quizzer_entry)?;
                    log::trace!(
                        "Quizzer: '{}' now has an average of '{}'",
                        quizzer_stats.name,
                        quizzer_stats.avg()
                    );
                }
                None => {
                    let quizzer_stats: QuizzerStats = quizzer_entry.into();
                    log::trace!(
                        "Quizzer: '{}' created with initial average of '{}'",
                        quizzer_stats.name,
                        quizzer_stats.avg()
                    );
                    self.quizzers
                        .insert(quizzer_stats.name.clone(), quizzer_stats);
                }
            }
        }
        Ok(())
    }
}

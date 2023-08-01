use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::name::{QuizzerName, TeamName};
use crate::quiz::Quiz;
use crate::stats::quizzer::QuizzerStats;
use crate::stats::team::TeamStats;
use crate::stats::Stats;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StatsRecord {
    pub teams: HashMap<TeamName, TeamStats>,
    pub quizzers: HashMap<QuizzerName, QuizzerStats>,
}

impl StatsRecord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, quiz: Quiz) -> Result<()> {
        for team_entry in quiz.team_entries {
            let team_stats = self
                .teams
                .entry(team_entry.name.clone())
                .or_insert_with(|| TeamStats::new(team_entry.name.clone()));

            team_stats.update(team_entry)?;
            log::trace!(
                "Team: '{}' now has an average of '{}'",
                team_stats.name,
                team_stats.avg()
            );
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

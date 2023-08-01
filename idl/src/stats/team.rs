use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::quiz::{QuizName, Team, TeamEntry, TeamName};
use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TeamStats {
    pub name: TeamName,
    pub quizzes: HashMap<QuizName, TeamEntry>,
}

impl Stats<TeamEntry> for TeamStats {
    fn update(&mut self, entry: TeamEntry) -> Result<()> {
        if entry.name != self.name {
            bail!("Name must be the same for stats: '{self:?}' and entry: '{entry:?}'");
        }
        self.quizzes.insert(entry.quiz.clone(), entry);
        Ok(())
    }

    fn avg(&self) -> f32 {
        self.points() as f32 / self.quizzes.len() as f32
    }

    fn tie_breaker(&self) -> f32 {
        self.score() as f32 / self.quizzes.len() as f32
    }
}

impl From<TeamEntry> for TeamStats {
    fn from(value: TeamEntry) -> Self {
        let name = value.name.clone();
        let mut quizzes = HashMap::new();
        quizzes.insert(value.quiz.clone(), value);
        Self { name, quizzes }
    }
}

impl Team for TeamStats {
    fn score(&self) -> i32 {
        self.quizzes.values().map(Team::score).sum()
    }
    fn points(&self) -> i32 {
        self.quizzes.values().map(Team::points).sum()
    }
    fn errors(&self) -> i32 {
        self.quizzes.values().map(Team::errors).sum()
    }
}

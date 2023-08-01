use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::quiz::{Quizzer, QuizzerEntry};
use crate::stats::error::StatsError;
use crate::stats::Stats;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct QuizzerStats {
    pub name: String,
    pub quizzes: HashMap<String, QuizzerEntry>,
}

impl Stats<QuizzerEntry> for QuizzerStats {
    fn update(&mut self, entry: QuizzerEntry) -> Result<(), StatsError> {
        if entry.name != self.name {
            return Err(StatsError::BadName {
                stats: self.name.clone(),
                entry: format!("{entry:?}"),
            });
        }

        self.quizzes.insert(entry.quiz.clone(), entry);
        Ok(())
    }

    fn avg(&self) -> f32 {
        self.points() as f32 / self.quizzes.len() as f32
    }

    fn tie_breaker(&self) -> f32 {
        if self.errors() > 0 {
            1. / self.errors() as f32
        } else {
            10.
        }
    }
}

impl From<QuizzerEntry> for QuizzerStats {
    fn from(value: QuizzerEntry) -> Self {
        let name = value.name.clone();
        let mut quizzes = HashMap::new();
        quizzes.insert(value.quiz.clone(), value);
        Self { name, quizzes }
    }
}

impl Quizzer for QuizzerStats {
    fn points(&self) -> i32 {
        self.quizzes.values().map(Quizzer::points).sum()
    }
    fn errors(&self) -> i32 {
        self.quizzes.values().map(Quizzer::errors).sum()
    }
    fn jumps(&self) -> i32 {
        self.quizzes.values().map(Quizzer::jumps).sum()
    }
    fn refer(&self) -> i32 {
        self.quizzes.values().map(Quizzer::refer).sum()
    }
    fn ftv(&self) -> i32 {
        self.quizzes.values().map(Quizzer::ftv).sum()
    }
    fn int(&self) -> i32 {
        self.quizzes.values().map(Quizzer::int).sum()
    }
    fn ma(&self) -> i32 {
        self.quizzes.values().map(Quizzer::ma).sum()
    }
    fn q(&self) -> i32 {
        self.quizzes.values().map(Quizzer::q).sum()
    }
    fn sit(&self) -> i32 {
        self.quizzes.values().map(Quizzer::sit).sum()
    }
}

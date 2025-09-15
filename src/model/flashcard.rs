use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum FlashCardError {
    #[error("Pytanie w fiszce nie może być puste.")]
    EmptyQuestion,
    #[error("Odpowiedź w fiszce nie może być pusta.")]
    EmptyAnswer,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlashCard {
    id: Uuid,
    question: String,
    answer: String,
    ef: f32,
    repetitions: u32,
    interval: u32,
    last_review_date: NaiveDate,
    next_review_date: NaiveDate,
}
impl FlashCard {
    pub fn new(question: String, answer: String, today: NaiveDate) -> Result<Self, FlashCardError> {
        if question.trim().is_empty() {
            return Err(FlashCardError::EmptyQuestion);
        }
        if answer.trim().is_empty() {
            return Err(FlashCardError::EmptyAnswer);
        }
        Ok(FlashCard {
            id: Uuid::new_v4(),
            question,
            answer,
            ef: 2.5,
            repetitions: 0,
            interval: 0,
            last_review_date: today,
            next_review_date: today,
        })
    }
    pub fn review(&mut self, quality: u32, today: NaiveDate) {
        let q = quality as f32;
        let mut new_ef = self.ef + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02));
        new_ef = new_ef.max(1.3);
        if quality < 3 {
            self.repetitions = 0;
            self.interval = 1;
        } else {
            self.interval = match self.interval {
                0 => 1,
                1 => 6,
                _ => (self.interval as f32 * self.ef).round() as u32,
            };
            self.ef = new_ef;
            self.repetitions += 1;
        }
        self.last_review_date = today;
        self.next_review_date = today + Duration::days(self.interval as i64);
    }
    pub fn get_date(&self) -> NaiveDate {
        self.next_review_date
    }
    pub fn get_question(&self) -> String {
        self.question.clone()
    }
    pub fn get_answer(&self) -> String {
        self.answer.clone()
    }
}

use super::flashcard::*;
use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
#[derive(Deserialize)]
struct RawCard {
    question: String,
    answer: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    name: String,
    cards: Vec<FlashCard>,
}
impl Deck {
    pub fn new(name: String) -> Self {
        Self {
            name,
            cards: vec![],
        }
    }

    pub fn new_from_file(path: &str, name: String, today: NaiveDate) -> Result<Deck> {
        let json_path = Path::new(path);

        let file = File::open(json_path)
            .with_context(|| format!("Nie udało się otworzyć pliku: {}", path))?;

        let reader = BufReader::new(file);

        let rawcards: Vec<RawCard> =
            serde_json::from_reader(reader).context("Błąd podczas parsowania pliku JSON.")?;
        let mut deck: Deck = Deck::new(name);
        for rawcard in rawcards {
            deck.add_card(FlashCard::new(rawcard.question, rawcard.answer, today)?);
        }
        Ok(deck)
    }
    pub fn add_card(&mut self, card: FlashCard) {
        self.cards.push(card);
    }

    pub fn get_review_count(&self, today: NaiveDate) -> usize {
        self.get_cards_to_review_indices(today).len()
    }
    pub fn get_cards_to_review_indices(&self, today: NaiveDate) -> Vec<usize> {
        self.cards
            .iter()
            .enumerate()
            .filter(|(_, el)| (*el).get_date() <= today)
            .map(|(i, _)| i)
            .collect()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_card_count(&self) -> usize {
        self.cards.len()
    }
    pub fn get_card(&self, index: usize) -> Option<&FlashCard> {
        self.cards.get(index)
    }
    pub fn get_card_mut(&mut self, index: usize) -> Option<&mut FlashCard> {
        self.cards.get_mut(index)
    }
}

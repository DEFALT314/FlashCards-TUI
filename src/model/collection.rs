use crate::model::deck::Deck;
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::path::Path;

const FILENAME: &'static str = "./deck.json";
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection {
    pub decks: Vec<Deck>,
}

impl Collection {
    pub fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string(self)?;
        let path = Path::new(FILENAME);
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    pub fn new() -> Result<Collection> {
        match File::open(FILENAME) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let collection: Collection = serde_json::from_reader(reader)
                    .context("Błąd podczas parsowania pliku JSON.")?;
                Ok(collection)
            }
            Err(error) => {
                if error.kind() == io::ErrorKind::NotFound {
                    Ok(Collection::default())
                } else {
                    Err(anyhow!(error))
                }
            }
        }
    }

    pub fn add_deck(&mut self, deck: Deck) {
        self.decks.push(deck);
    }
}

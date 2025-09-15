use crate::model::{Collection, Deck};
use ratatui::widgets::ListState;
#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Studying,
    AddingDeck,
    Exiting,
}
pub enum CurrentlyEditing {
    Name,
    Path,
}
pub struct StudyState {
    pub deck_index: usize,
    pub card_index: usize,
    pub is_answer_visible: bool,
    pub indexes: Vec<usize>,
}
pub struct App {
    pub name_input: String,
    pub path_input: String,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub deck_list_state: ListState,
    pub collection: Collection,
    pub study_state: Option<StudyState>,
}
impl App {
    pub fn new(collection: Collection) -> App {
        let mut deck_list_state = ListState::default();
        if !collection.decks.is_empty() {
            deck_list_state.select(Some(0));
        }
        App {
            name_input: String::new(),
            path_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            deck_list_state,
            collection,
            study_state: None,
        }
    }

    pub fn select_next(&mut self) {
        self.deck_list_state.select_next();
    }
    pub fn select_previous(&mut self) {
        self.deck_list_state.select_previous();
    }
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Name => self.currently_editing = Some(CurrentlyEditing::Path),
                CurrentlyEditing::Path => self.currently_editing = Some(CurrentlyEditing::Name),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Name);
        }
    }
    pub fn toggle_study(&mut self) {
        if let Some(idx) = self.deck_list_state.selected() {
            if let Some(deck) = self.collection.decks.get(idx) {
                let today = chrono::Local::now().naive_local().date();
                let indices = deck.get_cards_to_review_indices(today);
                if indices.is_empty() {
                    return;
                }
                self.study_state = Some(StudyState {
                    deck_index: idx,
                    card_index: 0,
                    is_answer_visible: false,
                    indexes: indices,
                });
                self.current_screen = CurrentScreen::Studying;
            }
        }
    }

    pub fn toggle_answer(&mut self) {
        if let Some(state) = &mut self.study_state {
            state.is_answer_visible = !state.is_answer_visible;
        }
    }
    pub fn rate_current_card(&mut self, rating: u32) {
        let today = chrono::Local::now().naive_local().date();
        if let Some(state) = &mut self.study_state {
            let idx = state.indexes[state.card_index];
            if let Some(card) = self.collection.decks[state.deck_index].get_card_mut(idx) {
                card.review(rating, today)
            }
            if state.card_index >= state.indexes.len() - 1 {
                self.stop_studying()
            } else {
                state.card_index += 1;
                state.is_answer_visible = false;
            }
        }
    }
    pub fn stop_studying(&mut self) {
        self.study_state = None;
        self.current_screen = CurrentScreen::Main;
    }
    pub fn add_new_deck(&mut self) {
        let path = self.path_input.trim().to_string();
        let name = self.name_input.trim().to_string();
        if let Ok(deck) =
            Deck::new_from_file(&path, name.clone(), chrono::Local::now().date_naive())
        {
            self.collection.add_deck(deck);
        }
        self.name_input.clear();
        self.path_input.clear();
        self.current_screen = CurrentScreen::Main;
    }
}

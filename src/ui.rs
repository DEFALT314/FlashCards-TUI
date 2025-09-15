use crate::app::{App, CurrentScreen, CurrentlyEditing};
use chrono::Utc;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Modifier, Span, Style};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Wrap};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

    match app.current_screen {
        CurrentScreen::Main => draw_main_menu(frame, app, chunks[0]),
        CurrentScreen::Studying => draw_study_view(frame, app, chunks[0]),
        _ => draw_main_menu(frame, app, chunks[0]),
    }

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "Nawigacja: ↑↓ | Enter: Ucz się | 'a': Dodaj talię | 'q': Wyjdź",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::AddingDeck => Span::styled(
                "Enter: Zatwierdź | Esc: Anuluj | Tab: Zmień pudełko",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Studying => Span::styled(
                "Space: Pokaż odpowiedź | 1-5: Oceń | Esc: Powrót",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled("", Style::default().fg(Color::Red)),
        }
    };
    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .centered()
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        );
    frame.render_widget(key_notes_footer, chunks[1]);
    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Wprowadź nazwę i ścieżkę")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("Nazwa").borders(Borders::ALL);
        let mut value_block = Block::default().title("Ścieżka").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightBlue).fg(Color::Black);

        match editing {
            CurrentlyEditing::Name => key_block = key_block.style(active_style),
            CurrentlyEditing::Path => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.name_input.clone()).block(key_block);
        frame.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.path_input.clone()).block(value_block);
        frame.render_widget(value_text, popup_chunks[1]);
    }
}

fn draw_study_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(90),
            // Constraint::Percentage(60),
            Constraint::Percentage(10),
        ])
        .split(area);
    let card_area = vertical_chunks[0];

    if let Some(state) = &app.study_state {
        if let Some(deck) = app.collection.decks.get(state.deck_index) {
            if let Some(card) = deck.get_card(state.indexes[state.card_index]) {
                let title: Line = Line::from(vec!["Uczysz się: ".into(), deck.get_name().into()])
                    .centered()
                    .style(Style::default().fg(Color::Red));
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(0, 0, card_area.height / 2, 0));
                let mut text_lines: Vec<Span> = vec![];
                if state.is_answer_visible {
                    text_lines.push("Odpowiedź: ".bold().bold().green());
                    text_lines.push(card.get_answer().green().into());
                } else {
                    text_lines = vec![
                        "Pytanie: \
                        "
                        .bold(),
                        "\n".into(),
                        card.get_question().into(),
                        "\n".into(),
                    ];
                }
                let card_paragraph = Paragraph::new::<Line>(text_lines.into())
                    .block(block)
                    .wrap(Wrap { trim: true })
                    .centered();

                frame.render_widget(card_paragraph, card_area);
                let block_info = Block::default()
                    .borders(Borders::NONE)
                    .padding(Padding::new(0, 0, vertical_chunks[1].height / 2, 0));
                let current_card_num: Span = (state.card_index + 1).to_string().into();
                let all_card_num: Span = (state.indexes.len()).to_string().into();
                let cards_paragraph_info = Paragraph::new::<Line>(
                    vec!["Karta: ".into(), current_card_num, "/".into(), all_card_num].into(),
                )
                .block(block_info)
                .centered();
                frame.render_widget(cards_paragraph_info, vertical_chunks[1]);
            }
        }
    }
}

fn draw_main_menu(frame: &mut Frame, app: &mut App, area: Rect) {
    let deck_items: Vec<ListItem> = app
        .collection
        .decks
        .iter()
        .map(|d| {
            ListItem::new(format!(
                "{:<25} ({} kart) ({} kart do powtórki)",
                d.get_name(),
                d.get_card_count(),
                d.get_review_count(Utc::now().date_naive())
            ))
        })
        .collect();

    let deck_list = List::new(deck_items)
        .block(
            Block::default()
                .title(Line::raw("Twoje talie").centered())
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(deck_list, area, &mut app.deck_list_state);
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

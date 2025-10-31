# FlashCards-TUI 📝

**FlashCards-TUI** is a terminal-based TUI application for learning flashcards, inspired by tools like Anki. It is designed to help users efficiently memorize and review concepts using spaced repetition.

The application is written in **Rust** and provides an interactive terminal interface for managing and studying flashcards.

## Features

- **Interactive Terminal UI**  
  Built with the [ratatui](https://crates.io/crates/ratatui) library for a responsive and user-friendly terminal interface.

- **Add New Flashcards from JSON Files**  
  - Press **`a`** in the main menu to open a prompt.  
  - Enter the deck name and the path to a JSON file containing new flashcards.  
  - Supports loading multiple flashcards at once.

- **Spaced Repetition Algorithm (SM-2)**  
  - Uses the SM-2 algorithm to calculate the optimal review date for each flashcard.  
  - Automatically selects flashcards that are due for review in study mode.

- **Save Progress Automatically**  
  - The application keeps your decks and learning progress across sessions, so you can continue where you left off.

## Usage

1. **Run the application** from your terminal:

```bash
cargo run --release
````

2. **Main menu shortcuts:**

* Press **`a`** → Add new flashcards from a JSON file
* Press **`q`** → Quit the application
* Use the arrow keys to navigate between decks and flashcards in study mode.

3. **Study Mode**
   The app automatically selects flashcards that are due for review based on the SM-2 algorithm. After reviewing, the flashcard’s next review date is recalculated.


## JSON Format for Flashcards

Each JSON file should contain an array of flashcards with the following structure:

```json
[
  {
    "question": "What is Rust?",
    "answer": "A systems programming language focused on safety and performance."
  },
  {
    "question": "What is SM-2?",
    "answer": "An algorithm used for spaced repetition in flashcard learning."
  }
]
```


## Installation

1. Install **Rust** (if not already installed) from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the repository:

```bash
git clone https://github.com/YourUsername/FlashCards-TUI.git
cd FlashCards-TUI
```

3. Build and run the application:

```bash
cargo build --release
cargo run --release
```



## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


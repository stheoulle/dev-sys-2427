# Project Architecture: TicTacToe Game

## Project Definition

This project is a command-line TicTacToe game implemented in Rust. The goal is to provide a simple, interactive experience where a human player competes against an AI opponent. The project demonstrates basic game logic, user input handling, and a simple AI using the minimax algorithm.

## Components and Modules

- **main.rs**: The entry point of the application. It manages the game loop, user interaction, and orchestrates calls to other functions.
- **Board Management**: Functions for creating, displaying, and updating the game board. These ensure the board state is always valid and user-friendly.
- **User Input**: Functions to handle player character selection and move input, including validation to prevent invalid moves.
- **AI Logic**: Implements the minimax algorithm for the AI to choose optimal moves, making the game challenging for the player.
- **Win/Draw Detection**: Functions to check for win conditions and detect draws, ensuring the game ends appropriately.

This modular architecture separates concerns, making the code easier to maintain and extend. Each module has a clear responsibility, which improves readability and testability.

## Usage

### How to Run

1. Ensure you have Rust installed.
2. In the project directory, run:

   ```bash
   cargo run
   ```

3. Follow the on-screen prompts to play against the AI.

### Example Session

```bash
[*] WELCOME TO TICTACTOE GAME [*]
Choose your character (X/O): X
┌───┬───┬───┐
│   │   │   │
├───┼───┼───┤
│   │   │   │
├───┼───┼───┤
│   │   │   │
└───┴───┴───┘
[+] Your move X -> (1-9)?: 5
... (game continues)
```

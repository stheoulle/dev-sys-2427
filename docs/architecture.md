# Project Architecture: TicTacToe Game

## Project Definition

This project is a command-line TicTacToe game implemented in Rust. The goal is to provide a simple, interactive experience where a human player competes against an AI opponent. The project demonstrates basic game logic, user input handling, and a simple AI using the minimax algorithm.

## Components and Modules

- **main.rs**: The entry point of the application. It only calls the main game function from the library.
- **lib.rs**: Contains all core game logic, including board management, user input, AI (minimax), win/draw detection, and utility functions. This separation allows for easier testing and reuse.
- **tests/game_tests.rs**: Integration tests for the main game logic, ensuring correctness of board operations, win/draw detection, and utility functions. Tests are written using Rust's standard test framework and import functions from `lib.rs`.

This architecture follows Rust best practices by separating the executable entry point from the library code, and by keeping tests in a dedicated folder. It improves maintainability, testability, and clarity.

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

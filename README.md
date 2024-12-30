# Guess the Number Game in Rust

## Overview

This project is an interactive "Guess the Number" game built with Rust. The game challenges the player to guess a randomly generated number between 1 and 10. The program provides helpful hints, such as whether the target number is higher or lower, and keeps track of the number of attempts it takes to guess correctly.

## Features

- **Random Number Generation**: Uses Rust's `rand` crate to generate a random number for each game session.
- **Hint System**: Guides the user with messages like "The number is higher" or "The number is lower" for incorrect guesses.
- **Replay Option**: Players can choose to play multiple rounds without restarting the program.
- **Custom Input Handling**: Includes a reusable `input` module for prompt-based input and error handling.

## How It Works

1. The program generates a random number between 1 and 10.
2. The user is prompted to guess the number.
3. If the guess is incorrect:
   - The program provides a hint (e.g., whether the number is higher or lower).
   - The user can guess again.
4. When the guess is correct, the program displays the number of attempts taken.
5. After each round, the player is asked whether they want to play again.

## Input Module

The `input` module is a reusable Rust module for handling user input. It provides two functions:

- `get_input`: Prompts the user for input and returns a trimmed string.
- `number_check`: Validates the input and parses it into an integer, defaulting to 0 if the input is invalid.

### Example Usage

```rust
let user_input = get_input("Enter a number: ");
let number = number_check(&user_input);

```

## Prerequisites

- Rust and Cargo must be installed. [Install Rust here](https://www.rust-lang.org/tools/install).

## Running the program

1. Clone the repository:

```sh
git clone https://github.com/Obcodelab/guessit.git
```

2. Navigate to the project directory:

```sh
cd guessit
```

3. Run the program:

```sh
cargo run
```

## Example Game Session

```vbnet
Guess the number: 5
Try again!
The number is higher
Guess the number: 8
You guessed correctly! in 2 tries
Do you want to play again? (y/n): y
Guess the number: 4
Try again!
The number is lower
Guess the number: 2
You guessed correctly! in 2 tries
Do you want to play again? (y/n): n
Thanks for playing!

```

## Possible Enhancements

- Add difficulty levels with adjustable number ranges.
- Include a scoring system to track performance across multiple rounds.
- Implement a graphical or web-based interface for the game.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request with suggestions or improvements.

# Word Guesser
Can you guess the 6-letter word in 5 tries or less?

## Running
* `git clone` repo
* `cd word_guesser`
* `cargo run` (or, `cargo build` and `./target/debug/word_guess` as often as you want to play that build)

## Helpfuly Tips
* The word list is an English, 6-letter word list with only basic alphabet characters (e.g. ASCII 97 to 122).
* Letters from your guess that are in the right position will be GREEN
* Letters that are in the word but in the wrong position will be YELLOW (note: the letter could already be used and accounted for elsewhere)
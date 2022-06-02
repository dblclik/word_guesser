use rand::Rng;
use std::io;
use colored::*;
use easy_player::single_player::SinglePlayer;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value_t = 1)]
    lives: usize,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 5)]
    guesses: usize,
}

struct GuessResult {
    guess_bytes: Vec<u8>,
    result_vector: Vec<u8>
}

struct PlayerUpdate {
    score_added: usize,
    score_reduced: usize,
    lives_added: usize,
    lives_reduced: usize
}

fn main() {
    let args = Args::parse();
    let my_str = include_str!("data/words.txt");
    let string: Vec<Vec<&str>> = my_str.split('\n')
        .map(|x: &str| x.split(' ').collect())
        .collect();

    let mut player = SinglePlayer{score: 0, lives: args.lives, credits: 1};
    while player.lives > 0 {
        println!();        
        println!("{}", "#########################".yellow());
        println!("{}", "# Time to play a game!! #".yellow());
        println!("{}", "#########################".yellow());
        println!();
        let update_values = word_guess(&string, args.guesses);
        player.add_lives(update_values.lives_added);
        player.reduce_lives(update_values.lives_reduced);
        player.add_score(update_values.score_added);
        player.reduce_score(update_values.score_reduced);
    }

    let ending_score = format!("You finished with {} points!!", player.score);
    println!("{}", ending_score.green().bold());
    
}

fn word_guess(string: &Vec<Vec<&str>>, number_of_guesses: usize) -> PlayerUpdate {   

    let row_chosen = rand::thread_rng().gen_range(0..string.len());
    let word_chosen = rand::thread_rng().gen_range(0..string[row_chosen].len());
    // println!("Chosen word is: {}", string[row_chosen][word_chosen]);
    let word_bytes = string[row_chosen][word_chosen].to_owned().into_bytes();
    let mut guesses = number_of_guesses;

    loop {
        let guess_fmt = guesses.to_string().red();
        let notification = "Please input your guess.".green().bold();
        println!("{} ({} {})", notification, guess_fmt, "guesses remaining".red());

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess.truncate(guess.len() - 1);
        let lowercase_guess = guess.to_lowercase();
        if !is_valid(&lowercase_guess) {
            println!("Guess must be a length 6 string with A-Z/a-z characters (mixed case is permitted)!");
            continue;
        }
        println!("You guessed: {}", lowercase_guess);

        let lowercase_guess_bytes = lowercase_guess.into_bytes();
        // println!("You guessed (in bytes): {:?}", lowercase_guess_bytes);

        if lowercase_guess_bytes.len() != 6 {
            println!("Required guess length is 6, your input was length {}!", lowercase_guess_bytes.len());
            continue;
        }
        guesses -= 1;
        let (win, result) = check_winning(&lowercase_guess_bytes, &word_bytes);
        format_guess_result(&result);

        if win {
            let win_message = format!("You win and earned {} points!", 20*(guesses+1));
            println!("{}", win_message.cyan().bold());
            return PlayerUpdate{score_added: 20*(guesses+1), score_reduced: 0, lives_added: 0, lives_reduced: 0}
        }
        else {
            if guesses <= 0 {
                let response = "You Lost... :(".cyan().bold();
                println!("{}", response);
                let correct_word_message = format!("The correct word was {}", string[row_chosen][word_chosen]);
                println!("{}", correct_word_message.cyan().bold());
                return PlayerUpdate{score_added: 0, score_reduced: 0, lives_added: 0, lives_reduced: 1}
            }
        }
    }
}

fn is_valid(s: &String) -> bool {
    let bytes = s.as_bytes();

    for (_, &item) in bytes.iter().enumerate() {
        if item < 97 || item > 122 {
            return false;
        }
    }

    return true
}

fn check_winning(guess: &Vec<u8>, word: &[u8]) -> (bool, GuessResult) {
    if guess == word {
        return (true, GuessResult{guess_bytes: guess.clone(), result_vector: vec![3,3,3,3,3,3]})
    }

    let mut result = vec![0,0,0,0,0,0];

    for (index, &item) in guess.iter().enumerate() {
        if &word[index] == &item {
            result[index] = 3;
        }
        else {
            if word.contains(&item) {
                result[index] = 1;
            }
        }
    }

    return (false, GuessResult{guess_bytes: guess.clone(), result_vector: result})

}

// This is gross, but it works for now
fn format_guess_result(result: &GuessResult) {
    let mut formatted_string_parts: [ColoredString; 6] = [ColoredString::default(),ColoredString::default(),ColoredString::default(),ColoredString::default(),ColoredString::default(),ColoredString::default()];
    for (index, &item) in result.guess_bytes.iter().enumerate() {
        let current_character = String::from_utf8(vec!(item));
            let current_character = match current_character {
                Ok(character) => character,
                Err(error) => panic!("Problem reading in the byte: {:?}", error)
            };
        formatted_string_parts[index] = match result.result_vector[index] {
            0 => current_character.bold(),
            1 => current_character.yellow().bold(),
            3 => current_character.green().bold(),
            _ => panic!("Problem matching the result: {}", result.result_vector[index])
        }
    }
    println!("{}{}{}{}{}{}", formatted_string_parts[0], formatted_string_parts[1], formatted_string_parts[2], formatted_string_parts[3], formatted_string_parts[4], formatted_string_parts[5]);
}
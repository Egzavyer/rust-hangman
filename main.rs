use std::io::{self, Write};

struct State {
    guess_vec: Vec<char>,
    remaining_letters: usize,
    remaining_errors: u8,
    num_words: u8,
}

fn main() {
    let mut word: Vec<char> = ask_word();
    let word_size: usize = word.len();
    let mut state = State {
        guess_vec: vec!['_'; word_size],
        remaining_letters: word_size,
        remaining_errors: 7,
        num_words: 1,
    };

    for i in 0..word_size {
        if word[i] == ' ' {
            state.guess_vec[i] = ' ';
            state.remaining_letters -= 1;
            state.num_words += 1;
        } else if word[i].is_alphanumeric() {
            println!("{}", word[i]);
            word[i] = word[i].to_ascii_lowercase();
            println!("{}", word[i]);
        }
    }

    while state.remaining_letters > 0 && state.remaining_errors > 0 {
        clear_terminal();
        println!("{}", draw_hangman(state.remaining_errors));
        print!("\n     ");
        for i in 0..word_size {
            print!("{} ", state.guess_vec[i]);
        }
        println!("\n\n{} Words", state.num_words);
        println!("{} Letters Remaining", state.remaining_letters);
        println!("{} Errors Remaining", state.remaining_errors);
        let guess: char = ask_guess();
        if !is_good_guess(&guess, &mut state, &word) {
            state.remaining_errors -= 1;
            println!("Wrong! '{}' is not in the word!", guess);
        } else {
            println!("Correct! '{}' is in the word!", guess);
        }
    }
    clear_terminal();
    println!("{}", draw_hangman(state.remaining_errors));
    if state.remaining_letters == 0 {
        println!("\tYou Won!");
    } else if state.remaining_errors == 0 {
        println!("\tYou Lose!");
    }
}

fn ask_word() -> Vec<char> {
    print!("Enter your word: ");
    let _ = io::stdout().flush();
    let mut word: String = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("ERROR: Unable to read stdin");
    let characters: Vec<char> = word.trim().chars().collect();
    return characters;
}

fn ask_guess() -> char {
    print!("Enter your guess: ");
    let _ = io::stdout().flush();
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("ERROR: Unable to read stdin");
    return guess.trim().chars().next().unwrap();
}

fn is_good_guess(guess: &char, state: &mut State, word: &Vec<char>) -> bool {
    let mut found = false;
    for i in 0..word.len() {
        if word[i] == *guess {
            found = true;
            state.remaining_letters -= 1;
            state.guess_vec[i] = *guess;
        }
    }
    return found;
}

fn clear_terminal() {
    print!("\x1b[2J\x1b[H");
}

fn draw_hangman(remaining_errors: u8) -> String {
    match remaining_errors {
        0 => {
            return "      ___
     |   ¦
     |   O
     |  /|\\
     |   |
     |  / \\"
                .to_string();
        }
        1 => {
            return "      ___
     |   ¦
     |   O
     |  /|\\
     |   |
     |  / "
                .to_string();
        }
        2 => {
            return "      ___
     |   ¦
     |   O
     |  /|\\
     |   |
     |  "
            .to_string();
        }
        3 => {
            return "      ___
     |   ¦
     |   O
     |  /|\\
     |   
     |  "
            .to_string();
        }
        4 => {
            return "      ___
     |   ¦
     |   O
     |  /|
     |   
     |  "
            .to_string();
        }
        5 => {
            return "      ___
     |   ¦
     |   O
     |  /
     |   
     |  "
            .to_string();
        }
        6 => {
            return "      ___
     |   ¦
     |   O
     |  
     |   
     |  "
            .to_string();
        }
        _ => {
            return "      ___
     |   ¦
     |   
     |  
     |   
     |  "
            .to_string();
        }
    }
}

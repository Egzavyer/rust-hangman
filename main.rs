use std::io::{self, Write};

fn main() {
    let word: Vec<char> = ask_word();
    let word_size: usize = word.len();
    let mut guess_vec: Vec<char> = vec!['_'; word.len()];
    let mut remaining_letters: usize = word_size;
    let mut remaining_errors: u8 = 7;

    //show empty slots for letters and hangman (7 tries)
    //  _ _
    // |   O
    // |  /|\
    // |   |
    // |  / \

    //ask for guess
    //find locations of the guessed letter in the word, add them to guess vector and display them on the slots
    //win when remaining letters = 0
    while remaining_letters > 0 && remaining_errors > 0 {
        println!("{}", draw_hangman(remaining_errors));
        print!("\n     ");
        for i in 0..word_size {
            print!("{} ", guess_vec[i]);
        }
        println!("\n\n{} Letters", word_size);
        println!("{} Errors Remaining", remaining_errors);
        let guess: char = ask_guess();
        if !is_good_guess(&guess, &mut guess_vec, &word) {
            remaining_errors = remaining_errors - 1;
            println!("Wrong! '{}' is not in the word!", guess);
        } else {
            remaining_letters = remaining_letters - 1;
            println!("Correct! '{}' is in the word!", guess);
        }
    }
    println!("{}", draw_hangman(remaining_errors));
    if remaining_letters == 0 {
        println!("You Won!");
    } else if remaining_errors == 0 {
        println!("You Lose!");
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

fn is_good_guess(guess: &char, guess_vec: &mut Vec<char>, word: &Vec<char>) -> bool {
    let mut found = false;
    for i in 0..word.len() {
        if word[i] == *guess {
            found = true;
            guess_vec[i] = *guess;
        }
    }
    return found;
}

fn draw_hangman(remaining_errors: u8) -> String {
    match remaining_errors {
        0 => {
            return "       _ _
     |   O
     |  /|\\
     |   |
     |  / \\"
                .to_string();
        }
        1 => {
            return "       _ _
     |   O
     |  /|\\
     |   |
     |  / "
                .to_string();
        }
        2 => {
            return "       _ _
     |   O
     |  /|\\
     |   |
     |  "
            .to_string();
        }
        3 => {
            return "       _ _
     |   O
     |  /|\\
     |   
     |  "
            .to_string();
        }
        4 => {
            return "       _ _
     |   O
     |  /|
     |   
     |  "
            .to_string();
        }
        5 => {
            return "       _ _
     |   O
     |  /
     |   
     |  "
            .to_string();
        }
        6 => {
            return "       _ _
     |   O
     |  
     |   
     |  "
            .to_string();
        }
        _ => {
            return "       _ _
     |   
     |  
     |   
     |  "
            .to_string();
        }
    }
}

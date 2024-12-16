use std::{fs::read_to_string, io::stdin};

use rand::{seq::SliceRandom, thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let words_string = read_to_string("./src/sevenletterwords.txt").unwrap();
    let mut words = words_string
        .split_whitespace()
        .map(|word| word.to_string())
        .collect::<Vec<String>>();
    let secret_index = rng.gen_range(0..words.len());
    let secret = words.remove(secret_index);

    words.shuffle(&mut rng);
    let mut guessables = get_words(&words, &secret);
    guessables.push(secret.clone());
    guessables.shuffle(&mut rng);

    guessables.iter().for_each(|word| {
        println!("{}", word);
    });

    for tries in (1..=4).rev() {
        println!(
            "Enter your guess of the password ({} tries remaining): ",
            tries
        );
        let player_move = get_input(&guessables);

        if player_move == *secret {
            println!("A C C E S S  G R A N T E D");
            return;
        } else {
            let match_count = count_letter_matches(&player_move, &secret);
            println!("Access Denied ({}/7 correct)", match_count);
        }
    }
    println!(
        "Locked out. Password reset. The secret password was {}",
        secret
    );
}

fn get_words(words: &Vec<String>, secret: &String) -> Vec<String> {
    let mut fake_secrets = vec![];
    let mut count_0_matches = 0;
    let mut count_1_matches = 0;
    let mut count_3_matches = 0;
    words
        .iter()
        .map(|word| (count_letter_matches(&word, &secret), word))
        .for_each(|(count, word)| {
            if count == 0 && count_0_matches != 2 {
                fake_secrets.push(word.to_owned());
                count_0_matches += 1;
            } else if count == 1 && count_1_matches != 7 {
                fake_secrets.push(word.to_owned());
                count_1_matches += 1;
            } else if count == 3 && count_3_matches != 2 {
                fake_secrets.push(word.to_owned());
                count_3_matches += 1;
            } else if fake_secrets.len() == 11 {
                return;
            }
        });
    if fake_secrets.len() != 11 {
        println!("Generating additional matches..."); //just for my own knowledge
        words.iter().for_each(|word| {
            if !fake_secrets.contains(&word) && fake_secrets.len() < 12 {
                fake_secrets.push(word.to_owned());
            }
        });
    }
    fake_secrets
}

fn count_letter_matches(word0: &str, word1: &str) -> i32 {
    let mut matches = 0;
    word0
        .char_indices()
        .zip(word1.char_indices())
        .for_each(|(left, right)| {
            if left.0 == right.0 && left.1 == right.1 {
                matches += 1;
            }
        });
    matches
}

fn get_input(words: &Vec<String>) -> String {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = &input.trim().to_lowercase();
        if words.contains(&input) {
            return input.to_owned();
        } else {
            println!("That's not one of the possible passwords listed.");
            println!("Try entering '{}' or '{}'", words[0], words[1]);
        }
    }
}

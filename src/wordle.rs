use rand::prelude::SliceRandom;
use std::error::Error;
use std::option::Option;
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;
use std::{fs, io, process};

#[derive(Debug)]
pub enum LetterType {
    Green,
    Yellow,
    Blank,
    Repeat, // Letters get marked as blank when already used.
            // I could adjust the code for this but this is a quick fix.
}

#[derive(Debug)]
pub struct Response {
    response: Vec<LetterType>,
}

impl FromStr for Response {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut response: Vec<LetterType> = Vec::new();
        for c in s.to_string().chars() {
            match c {
                '_' => response.push(LetterType::Blank),
                'Y' => response.push(LetterType::Yellow),
                'G' => response.push(LetterType::Green),
                'R' => response.push(LetterType::Repeat),
                _ => panic!("Invalid character: {c}."),
            }
        }
        Ok(Response { response })
    }
}

pub fn guess_candidates(word_list: &Vec<String>) -> Option<Vec<String>> {
    // We want words with large numbers of unique vowels, e.g. "Audio" has 4 unique vowels.
    // Knowing that words like "audio" exist, we can narrow down our search to words with at
    // least 4 unique vowels.
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut candidates: Vec<String> = Vec::new();
    // The word list might be empty, so we should return an option so we can return None.
    if word_list.len() == 0 {
        return None;
    }
    for word in word_list {
        let mut score: u8 = 0;
        for vowel in vowels {
            if word.contains(vowel) {
                score += 1;
            }
        }
        if score >= 4 {
            candidates.push(word.clone());
        }
    }
    Some(candidates)
}

pub fn prune_list(word_list: &Vec<String>, choice: &String, response: Response) -> Vec<String> {
    let cs = choice.chars().enumerate();
    let rs = response.response.iter();
    let mut prune_list = word_list.clone();
    for ((i, c), r) in cs.zip(rs) {
        match r {
            LetterType::Green => {
                prune_list = prune_list
                    .into_iter()
                    .filter(|x| x.contains(c))
                    .filter(|x| x.chars().nth(i).unwrap() == c)
                    .collect()
            }
            LetterType::Yellow => {
                prune_list = prune_list
                    .into_iter()
                    .filter(|x| x.contains(c))
                    .filter(|x| x.chars().nth(i).unwrap() != c)
                    .collect()
            }
            LetterType::Blank => {
                prune_list = prune_list.into_iter().filter(|x| !x.contains(c)).collect()
            }
            LetterType::Repeat => (),
        }
    }
    prune_list
}

pub fn choose(word_list: &Vec<String>) -> Option<String> {
    // TODO: Instead of choosing randomly, choose the word that eliminates the most possibilities.
    // This will be different per dictionary.
    match word_list.choose(&mut rand::thread_rng()) {
        Some(s) => Some(s.clone()),
        None => None,
    }
}

pub fn remove_word(word_list: &Vec<String>, word: &String) -> Vec<String> {
    let remove_list = word_list.clone();
    remove_list.into_iter().filter(|x| x != word).collect()
}

pub fn load_file_to_vec<T>(path: &Path) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let data: String = fs::read_to_string(path)?.parse()?;

    let contents: Vec<T> = data.lines().map(|x| x.parse::<T>().unwrap()).collect();

    Ok(contents)
}

pub fn run() {
    let data_path = Path::new("wordle_list.txt");
    let mut word_list = load_file_to_vec::<String>(data_path).unwrap();
    let guesses = guess_candidates(&word_list).unwrap();

    println!("===== Key =====");
    println!("GGGGG => Complete");
    println!("__GR_ => If a letter is marked blank but already correct e.g. 'green'.");
    println!("__G__ => Letter 3 is correct.");
    println!("__Y__ => Letter 3 is a valid letter but in the wrong place.");
    println!("_____ => All letters incorrect.");
    println!("ERROR => Word suggestion invalid.");

    let mut choice = guesses.choose(&mut rand::thread_rng()).unwrap().clone();

    loop {
        let possibilities = word_list.len();
        println!("Possibilities: {possibilities}");
        if possibilities <= 10 {
            println! {"{word_list:?}"};
        } else {
            println!("Guess: {choice}.");
        }

        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("main.rs - main(): Failed to read input.");
        response.truncate(5);
        match response.as_str() {
            "ERROR" => word_list = remove_word(&word_list, &choice),
            "GGGGG" => {
                println!("Quitting...");
                process::exit(0)
            }
            _ => {
                println!("Parsing response...");
                word_list = prune_list(&word_list, &choice, Response::from_str(&response).unwrap());
            }
        }

        choice = match choose(&word_list) {
            Some(s) => s,
            None => {
                println!("Word list exhausted. Quitting...");
                process::exit(0)
            }
        };
    }
}

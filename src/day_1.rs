
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::Iterator;
use std::string::ToString;

pub fn day_1a() {
    let file = File::open("data/day_1").unwrap();
    let reader = BufReader::new(file);

    let mut s = 0;
    for line in reader.lines() {
        let a = line.unwrap();

        let mut c_val = 0;

        let first_v = a.chars().find(|c|c.is_numeric()).unwrap().to_digit(10).unwrap();
        let last_v = a.chars().rev().find(|c|c.is_numeric()).unwrap().to_digit(10).unwrap();
        c_val += first_v * 10;
        c_val += last_v;
        s += c_val;
    }
    println!("{}", s)
}

pub static DIGIT_WORDS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
pub struct MatchingWord {
    buffer: String
}

impl Default for MatchingWord {
    fn default() -> Self {
        MatchingWord{buffer: "".to_string()}
    }
}

fn find_digit(a: &str, reverse: bool) -> u32 {
    let mut v = 0;
    let mut current_state = MatchingWord::default();
    let chars: Vec<char> = if reverse { a.chars().rev().collect() } else { a.chars().collect() };
    for c in chars {
        if c.is_numeric() {
            v = c.to_digit(10).unwrap();
            current_state = MatchingWord::default();
            break;
        }

        current_state.buffer.push(c);

        let mut full_match = false;

        /*
        here is where I got wrong
        When going through word check, there are 3 cases:
        1. found a full match, break and return
        2. found at least 1 partial match, continue collect buffer
        3. no partial match, here we should enter a loop of popping char off 1 by 1 from
        */
        loop {
            if current_state.buffer.is_empty() {
                break
            }
            let mut partial_match = false;
            for (idx, word) in DIGIT_WORDS.iter().enumerate() {
                let le_word: String = if reverse { word.chars().rev().collect() } else { word.chars().collect() };

                let cur = &current_state.buffer;

                full_match = *cur == le_word;
                if full_match {
                    v = idx as u32;
                    break;
                } else if le_word.starts_with(cur) {
                    partial_match = true;
                    break;
                }
            }
            if full_match || partial_match {
                break
            } else {
                let mut vc: Vec<char> = current_state.buffer.chars().collect();
                vc.remove(0);
                current_state.buffer = vc.iter().collect();
            }
        }
        if full_match {
            break
        }
    }
    v
}

pub fn day_1b() {
    /*
    given a string, start from index 0, read char by char
        for each char:
            if digit:
                reach
            else
            check current buffer:
                if empty:
                    take cur char, check if any digit word starts with char
                        if so:
                            update buffer
                        else:
                            pass
                else:
                    combine buffer with new char
                    check if updated string match digit word fully
                        if so:
                            reach conclusion
                        else:
                            pass

    */
    let file = File::open("data/day_1").unwrap();
    let reader = BufReader::new(file);

    let mut s = 0;
    for line in reader.lines() {
        let _current_state = MatchingWord::default();
        let a = line.unwrap();

        let first_v = find_digit(a.as_str(), false);
        let sec_v = find_digit(a.as_str(), true);

        let mut c_val = 0;
        c_val += first_v * 10;
        c_val += sec_v;
        s += c_val;
    }
    println!("{}", s)
}
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead};

use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, none_of};
use nom::combinator::{map_res};
use nom::branch::alt;
use nom::IResult;
use nom::multi::many1;

use nom::error::{Error, ErrorKind, ParseError};
use crate::day_3::Ele::Dots;

#[derive(Clone, Copy)]
struct IndexedStr<'a> (&'a str, u32);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Ele {
    Dots,
    Digit(u32),
    Symbol,
    Gear,
}

fn parse_dots(input:IndexedStr) -> IResult<IndexedStr, Ele, Error<& str>> {
    let (input_1, dots) = many1(tag("."))(input.0)?;
    Ok((IndexedStr(input_1,  input.1 + dots.len() as u32), Dots))
}

fn parse_symbol(input: IndexedStr) -> IResult<IndexedStr, Ele, Error<& str>> {
    let (input_1, _o) = none_of("*0123456789.")(input.0)?;
    Ok((IndexedStr(input_1, input.1 + 1), Ele::Symbol))
}

fn parse_gear(input: IndexedStr) -> IResult<IndexedStr, Ele, Error<& str>> {
    let (input_1, _o) = char('*')(input.0)?;
    Ok((IndexedStr(input_1, input.1 + 1), Ele::Gear))
}


fn parse_number(input: IndexedStr) -> IResult<IndexedStr, Ele, Error<& str>> {
    let (input_1, digit) = map_res(digit1, str::parse::<u32>)(input.0)?;
    let no_dec = digit.to_string().len();
    Ok((IndexedStr(input_1, input.1 + no_dec as u32), Ele::Digit(digit)))
}

impl <'a> ParseError<IndexedStr<'a>> for Error<&'a str> {
    fn from_error_kind(input: IndexedStr<'a>, kind: ErrorKind) -> Self {
        Error::new(input.0, kind)
    }

    fn append(_input: IndexedStr, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug)]
struct SingleLine{
    numbers: [Ele; 140],
    number_idx: Vec<u32>,
}

impl Default for SingleLine {
    fn default() -> Self {
        SingleLine{
            numbers: [Ele::Dots; 140],
            number_idx: vec![]
        }
    }
}

fn parse_single_line_a(input: &str) -> SingleLine {
    let mut remaining = IndexedStr(input, 0);
    let mut line_output = SingleLine::default();
    let mut no_id = 0;
    while !remaining.0.is_empty() {
        let (next, out) = alt((parse_dots, parse_symbol, parse_number))(remaining).unwrap();
        match out {
            Ele::Digit(d) => {
                line_output.number_idx.push(d);
                for i in 1..d.to_string().len() + 1 {
                    let idx: usize = next.1 as usize - i;
                    line_output.numbers[idx] = Ele::Digit(no_id)
                }
                no_id += 1;
            }
            Ele::Symbol => {
                line_output.numbers[(next.1 - 1) as usize] = Ele::Symbol
            }
            _ => {}
        }
        remaining = next;
    }

    line_output
}

fn parse_single_line_b(input: &str) -> SingleLine {
    let mut remaining = IndexedStr(input, 0);
    let mut line_output = SingleLine::default();
    let mut no_id = 0;
    while !remaining.0.is_empty() {
        let (next, out) = alt((parse_dots, parse_symbol, parse_number, parse_gear))(remaining).unwrap();
        match out {
            Ele::Digit(d) => {
                line_output.number_idx.push(d);
                for i in 1..d.to_string().len() + 1 {
                    let idx: usize = next.1 as usize - i;
                    line_output.numbers[idx] = Ele::Digit(no_id)
                }
                no_id += 1;
            }
            Ele::Gear => {
                line_output.numbers[(next.1 - 1) as usize] = Ele::Gear
            }
            _ => {}
        }
        remaining = next;
    }

    line_output
}
fn accumulate_2lines_a(a: &SingleLine, b: &SingleLine, line_no_of_b: usize) -> HashMap<String, u32> {
    let mut collected_digits: HashMap<String, u32> = HashMap::new();
    for (i, e) in b.numbers.iter().enumerate() {
        if *e == Ele::Symbol {
            for adjacent in a.numbers[i-1..i+2].iter() {
                match adjacent {
                    Ele::Digit(key) => {
                        let line_a = line_no_of_b - 1;
                        collected_digits.insert(format!("{line_a}_{key}"), a.number_idx[*key as usize]);
                    }
                    _ => {}
                }
            }
            match b.numbers[i - 1] {
                Ele::Digit(key) => {
                    collected_digits.insert(format!("{line_no_of_b}_{key}"), b.number_idx[key as usize]);
                }
                _ => {}
            }
            match b.numbers[i + 1] {
                Ele::Digit(key) => {
                    collected_digits.insert(format!("{line_no_of_b}_{key}"), b.number_idx[key as usize]);
                }
                _ => {}
            }
        }

        let prev_e = a.numbers[i];
        if prev_e == Ele::Symbol {
            for adjacent in b.numbers[i-1..i+2].iter() {
                match adjacent {
                    Ele::Digit(key) => {
                        collected_digits.insert(format!("{line_no_of_b}_{key}"), b.number_idx[*key as usize]);
                    }
                    _ => {}
                }
            }
        }
    }
    collected_digits
}

fn accumulate_3lines(a: &SingleLine, b: &SingleLine, c: &SingleLine, line_c: usize) -> u32 {
    let mut collected_ratios = 0;
    let line_a = line_c - 2;
    let line_b = line_c - 1;

    for (i, e) in b.numbers.iter().enumerate() {
        if *e == Ele::Gear {
            let mut collected_digits: HashMap<String, u32> = HashMap::new();
            for adjacent in a.numbers[i-1..i+2].iter() {
                match adjacent {
                    Ele::Digit(key) => {
                        collected_digits.insert(format!("{line_a}_{key}"), a.number_idx[*key as usize]);
                    }
                    _ => {}
                }
            }
            for adjacent in c.numbers[i-1..i+2].iter() {
                match adjacent {
                    Ele::Digit(key) => {
                        collected_digits.insert(format!("{line_c}_{key}"), c.number_idx[*key as usize]);
                    }
                    _ => {}
                }
            }

            match b.numbers[i - 1] {
                Ele::Digit(key) => {
                    collected_digits.insert(format!("{line_b}_{key}"), b.number_idx[key as usize]);
                }
                _ => {}
            }
            match b.numbers[i + 1] {
                Ele::Digit(key) => {
                    collected_digits.insert(format!("{line_b}_{key}"), b.number_idx[key as usize]);
                }
                _ => {}
            }

            if collected_digits.len() == 2 {
                let x: u32 = collected_digits.values().product();
                collected_ratios += x;
            }
        }
    }
    collected_ratios
}

pub fn day_3a() {
    /*
    fill in 2 matric
    */
    let file = File::open("data/day_3").unwrap();
    let _c = 0;
    let reader = BufReader::new(file);

    let mut prev = SingleLine::default();
    let mut final_r = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let cur = parse_single_line_a(line.unwrap().as_str());
        let x = accumulate_2lines_a(&prev, &cur, i);
        final_r.extend(x);
        prev = cur;
    }
    let ans: u32 = final_r.values().sum();
    println!("{:?}", ans);
}
pub fn day_3b() {
    /*
    fill in 2 matric
    */
    let file = File::open("data/day_3").unwrap();
    let mut ans = 0;
    let reader = BufReader::new(file);

    let mut line_a = SingleLine::default();
    let mut line_b = SingleLine::default();

    let mut line_no = 0;
    for (i, line) in reader.lines().enumerate() {
        let line_c = parse_single_line_b(line.unwrap().as_str());
        let x = accumulate_3lines(&line_a, &line_b, &line_c, i + 2);
        ans +=x;
        line_a = line_b;
        line_b = line_c;

        line_no = i + 2;
    }

    // one last loop to handle last line, special case because we are merging 3 line at a time and always do something like a look ahead
    let line_c = SingleLine::default();
    let x = accumulate_3lines(&line_a, &line_b, &line_c, line_no);
    ans += x;

    println!("{:?}", ans);
}

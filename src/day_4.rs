use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, multispace1, space0, space1};
use nom::combinator::{map_res};
use nom::{IResult, Parser};
use nom::multi::{ separated_list1};
use nom::sequence::tuple;

use num::BigInt;


#[derive(Clone, Copy, Debug)]
struct Hand {
    card_id: u32,
    winning: [u32;10],
    given: [u32;25],
}

impl Hand {
    fn winning_hands(self: &Hand) -> u32 {
        let mut x = 0;
        for i in self.given {
            if self.winning.contains(&i) {
                x+=1;
            }
        }
        x
    }
    fn points(self: &Hand) -> u32 {
        let mut x = 0;
        for i in self.given {
            if self.winning.contains(&i) {
                if x == 0 {
                    x = 1;
                } else {
                    x *= 2;
                }
            }
        }
        x

    }

    fn scratch_cards(self: &Hand, card_winning: &mut HashMap<u32, BigInt>) -> BigInt {
        let card_won = self.winning_hands();
        let mut no_of_card_won: BigInt  = BigInt::from(1);  // each card wins itself at the very least, thus start with 1
        for i in 0 ..card_won {
            let winning_card = i + self.card_id + 1;

            if let Some(won) = card_winning.get(&winning_card) { no_of_card_won += won; }
        }
        card_winning.insert(self.card_id, no_of_card_won.clone());
        no_of_card_won
    }
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_numbers<const C: usize>(input: &str) -> IResult<&str, [u32; C]> {
    let (i, _o) = space0(input)?;
    let (i, o) = separated_list1(space1, parse_number)(i)?;
    let out: [u32; C] = o.try_into().unwrap();
    Ok((i, out))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (_, _, id, _)) = tuple((tag("Card"),multispace1, parse_number, tag(":")))(input)?;

    let (hand, winning) = take_until("|")(input)?;
    let (hand, _) = tag("|")(hand)?;
    let h = Hand {
        card_id: id,
        winning: parse_numbers::<10>(winning)?.1,
        given: parse_numbers::<25>(hand)?.1,
    };
    Ok((hand, h))
}

pub fn day_4a() {
    /*
    fill in 2 matric
    */
    let file = File::open("data/day_4").unwrap();
    let reader = BufReader::new(file);

    let mut f = 0;
    for (_i, line) in reader.lines().enumerate() {
        let (_, hand) = parse_hand(line.unwrap().as_str()).unwrap();
        f += hand.points();
    }
    println!("{:?}", f);
}

pub fn day_4b() {
    /*
    fill in 2 matric
    */
    let file = File::open("data/day_4").unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<_> = reader.lines().map(|line| { line.unwrap() }).collect();
    lines.reverse();

    let mut card_won = HashMap::new();
    let mut res: BigInt = BigInt::from(0);
    for (_i, line) in lines.iter().enumerate() {
        let (_, hand) = parse_hand(line.as_str()).unwrap();
        res += hand.scratch_cards(&mut card_won);
    }
    println!("{:?}", res);
}



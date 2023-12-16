use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::Iterator;

use nom::{
    character::complete::{char, digit1, multispace0, space1},
    bytes::complete::tag,
    combinator::{map_res},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use nom::character::complete::alpha1;

struct Draw {
    r: u32,
    b: u32,
    g: u32,
}
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn power(&self) -> u32 {
        let mut min_draw = Draw {r:0, g:0,b:0};
        for d in self.draws.iter() {
            min_draw.r = min_draw.r.max(d.r);
            min_draw.g = min_draw.g.max(d.g);
            min_draw.b = min_draw.b.max(d.b);
        }
        min_draw.r * min_draw.g * min_draw.b
    }
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_color(input: &str) -> IResult<&str, u32> {
    preceded(multispace0, parse_number)(input)
}

fn parse_cube_count(input: &str) -> IResult<&str, (u32, &str)> {
    let (input, (count, _, color)) = tuple((parse_number, space1, alpha1))(input)?;
    Ok((input, (count, color)))
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    let (input, ccs) = separated_list1(tag(", "), parse_cube_count)(input)?;

    Ok((input, Draw {
        r: ccs.iter().find(|(_, color)|*color == "red").map(|(c, _)|*c).unwrap_or(0),
        g: ccs.iter().find(|(_, color)|*color == "green").map(|(c, _)|*c).unwrap_or(0),
        b: ccs.iter().find(|(_, color)|*color == "blue").map(|(c, _)|*c).unwrap_or(0),
    }))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (id, _, draws)) = tuple(
        (
        preceded(multispace0, preceded(tag("Game "), parse_number)),
        preceded(tag(":"), multispace0),
        separated_list1(tag("; "), parse_draw),
        )
    )(input)?;

    Ok((input, Game { id, draws }))
}

fn validate(truth: &Draw, draw: &Draw) -> bool {
    draw.r <= truth.r && draw.g <= truth.g && draw.b <= truth.b
}

pub fn day_2a() {
    let truth = Draw{r: 12, g: 13, b:14};
    let file = File::open("data/day_2").unwrap();
    let mut c = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        /*
        parse to draw, perform validation
        */
        let (_, game) = parse_game(line.unwrap().as_str()).unwrap();
        let mut game_valid = true;
        for d in game.draws {
            if !validate(&truth, &d) {
                game_valid = false;
                break;
            }
        }
        if game_valid {
            c += game.id;
        }
    }
    println!("{:?}", c)
}
pub fn day_2b() {
    let truth = Draw{r: 12, g: 13, b:14};
    let file = File::open("data/day_2").unwrap();
    let mut c = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        /*
        parse to draw, perform validation
        */
        let (_, game) = parse_game(line.unwrap().as_str()).unwrap();
        c += game.power();
    }
    println!("{:?}", c)
}

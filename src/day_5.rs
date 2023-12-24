use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, multispace0, space0, space1};
use nom::combinator::{map, map_res};
use nom::multi::{ separated_list1};
use nom::{IResult, Parser};
use nom::sequence::{preceded, tuple};

use crate::day_5::RangeOverlap::{Full, NotInRange, Partial};

#[derive(Debug)]
struct LeEntry {
    dest: u64,
    source: u64,
    range: u64
}

#[derive(Debug)]
struct LeMap {
    entries: Vec<LeEntry>
}

enum RangeOverlap {
    Full{dest_range: (u64, u64)},
    NotInRange{residual: (u64, u64)},
    Partial{dest_range: (u64, u64), residual: (u64, u64)}
}
impl LeEntry {
    fn compute_dest_range(&self, target_key: u64, target_range: u64) -> RangeOverlap {
        if self.source > target_key {
            // if the entry has past target, means no match, just return target range as is
            return Full {dest_range: (target_key, target_range)}
        }

        let source_offset = target_key - self.source;

        if target_key <= (self.source + self.range) {
            let dest_start =  source_offset + self.dest;
            let dest_in_range = self.range - source_offset;
            let dest_range = if target_range <= dest_in_range { target_range } else { dest_in_range };

            if dest_range < target_range {
                Partial {dest_range: (dest_start, dest_range), residual: (target_key + dest_range, target_range - dest_range)}
            } else {
                Full {dest_range: (dest_start, dest_range)}
            }
        } else {
            NotInRange {residual: (target_key, target_range)}
        }
    }
}

impl LeMap {
    fn find(&self, key: u64) -> u64 {
        match self.entries.iter().find(|e| key >= e.source && key < (e.source + e.range)) {
            None => {
                key
            }
            Some(entry) => {
                key - entry.source + entry.dest
            }
        }
    }

    fn find_range(&self, target: (u64, u64)) -> Vec<(u64, u64)> {
        let (key, range) = target;
        let mut search_start = 0;
        let mut search_end = self.entries.len() - 1;

        let mut i = (search_end + search_start) / 2; // floor
        loop {

            let cur = &self.entries[i];
            let next = &self.entries[i+1];

            if key >= cur.source && key <= next.source {
                break
            } else if key >= cur.source {
                search_start = i + 1
            } else {
                search_end = i
            }

            if search_end == search_start {
                i = search_end;
                break;
            }

            i = (search_end + search_start) / 2;
        }

        /*
        given a starting index and a starting target

        check if target start is out of range

        compute_dest_range
            full -> add the range to output, break
            partial -> add range to output, compute_dest_range with the next entry, update target
            none -> continue with next entry
        */
        let mut output: Vec<(u64, u64)> = vec![];
        let mut search_k = key;
        let mut search_range = range;

        loop {
            let entry = &self.entries[i];

            // this range does not exists in mapping
            if search_k > (entry.source + entry.range) {
                output.push((search_k, search_range));
                break;
            }

            match entry.compute_dest_range(search_k, search_range) {
                Full { dest_range } => {
                    output.push(dest_range);
                    break;
                }
                NotInRange { residual: _ } => {}
                Partial { dest_range, residual } => {
                    output.push(dest_range);
                    search_k = residual.0;
                    search_range = residual.1;
                }
            };

            i += 1;
        }
        output
    }
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    let (i, _o) = space0(input)?;
    let (i, o) = separated_list1(space1, parse_number)(i)?;
    Ok((i, o))
}

fn parse_init_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (o, _) = tag("seeds:")(input)?;
    let (o, seeds_s) = take_until("\n")(o)?;
    let (_, seeds) = parse_numbers(seeds_s)?;
    Ok((o, seeds))
}

fn parse_le_entry(input: &str) -> IResult<&str, LeEntry> {
    map(preceded(multispace0, parse_numbers), |nos| {
        LeEntry {
            dest: nos[0],
            source: nos[1],
            range: nos[2],
        }
    })(input)
}

fn parse_map<'a>(input: &'a str, expected_header: &'a str) -> IResult<&'a str, LeMap> {
    let header_part = preceded(multispace0, tag(expected_header));
    let (o, mut entries) = preceded(header_part, separated_list1(tag("\n"), parse_le_entry)).parse(input)?;

    entries.sort_by(|a, b|a.source.partial_cmp(&b.source).unwrap());

    Ok((o, LeMap{entries }))
}

pub fn day_5a() {
    let file = File::open("data/day_5").unwrap();
    let reader = BufReader::new(file);
    let string: String = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>().join("\n");
    let all_txt: & str = string.as_str();

    let (_, (seeds, seed_soil, soil_fert, fert_water, water_light, light_temp, temp_humid, humid_loc)) =
        tuple((parse_init_seeds,
           |s|parse_map(s, "seed-to-soil map:"),
           |s|parse_map(s, "soil-to-fertilizer map:"),
           |s|parse_map(s, "fertilizer-to-water map:"),
           |s|parse_map(s, "water-to-light map:"),
           |s|parse_map(s, "light-to-temperature map:"),
           |s|parse_map(s, "temperature-to-humidity map:"),
           |s|parse_map(s, "humidity-to-location map:"),
    ))(all_txt).unwrap();

    let locs = seeds.iter().map(|s| {
        let soil = seed_soil.find(*s);
        let fert = soil_fert.find(soil);
        let water = fert_water.find(fert);
        let light = water_light.find(water);
        let temp = light_temp.find(light);
        let humid = temp_humid.find(temp);
        
        humid_loc.find(humid)
    });

    let smallest = locs.min().unwrap();

    println!("{:?}", smallest)
}
pub fn day_5b() {
    let file = File::open("data/day_5").unwrap();
    let reader = BufReader::new(file);
    let string: String = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>().join("\n");
    let all_txt: & str = string.as_str();

    let (_, (seeds, seed_soil, soil_fert, fert_water, water_light, light_temp, temp_humid, humid_loc)) =
        tuple((parse_init_seeds,
               |s|parse_map(s, "seed-to-soil map:"),
               |s|parse_map(s, "soil-to-fertilizer map:"),
               |s|parse_map(s, "fertilizer-to-water map:"),
               |s|parse_map(s, "water-to-light map:"),
               |s|parse_map(s, "light-to-temperature map:"),
               |s|parse_map(s, "temperature-to-humidity map:"),
               |s|parse_map(s, "humidity-to-location map:"),
        ))(all_txt).unwrap();

    /*

    */

    let seeds_range: Vec<&[u64]> = seeds.as_slice().windows(2).collect();

    let mut locs = vec![];
    for seed_r in seeds_range {
        let s = seed_r[0];
        let r = seed_r[1];

        let final_locs: Vec<(u64, u64)> = seed_soil.find_range((s, r))
            .iter().flat_map(|i| soil_fert.find_range(*i))
            .flat_map(|i| fert_water.find_range(i))
            .flat_map(|i| water_light.find_range(i))
            .flat_map(|i| light_temp.find_range(i))
            .flat_map(|i| temp_humid.find_range(i))
            .flat_map(|i| humid_loc.find_range(i)).collect();

        let m = final_locs.iter().map(|(s, _r)|*s).min();
        locs.push(m.unwrap());
    }
    let smallest = locs.iter().min().unwrap();

    println!("{:?}", smallest)
}

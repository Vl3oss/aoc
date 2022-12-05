use std::collections::hash_map::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!("Unknown char: {}", c),
    }
}

fn part1(rucksacks: Vec<String>) -> u32 {
    let mut total = 0;

    for rs in rucksacks {
        let mut compart1 = HashMap::new();
        let mut compart2 = HashMap::new();
        println!("input = {}", rs);

        let len = rs.len();
        rs.chars().enumerate().for_each(|(i, c)| {
            if i < len / 2 {
                compart1.insert(c, 1);
            } else {
                compart2.insert(c, 1);
            }
        });
        println!(
            "compart1 = {:?}, compart2 = {:?}",
            compart1.keys(),
            compart2.keys()
        );

        for c in compart2.keys() {
            if compart1.contains_key(&c) {
                println!("compart1 contains {}", c);
                total += to_priority(*c);
            }
        }
    }

    total
}

fn part2(rucksacks: Vec<String>) -> u32 {
    let mut total = 0;

    for rucks in rucksacks.chunks(3) {
        let mut elves = [HashMap::new(), HashMap::new(), HashMap::new()];
        println!("input = {:?}", rucks);

        rucks.iter().enumerate().for_each(|(i, rs)| {
            rs.chars().for_each(|c| {
                elves[i].insert(c, 1);
            })
        });

        println!(
            "elf1 = {:?}, elf2 = {:?}, elf3 = {:?}",
            elves[0], elves[1], elves[2],
        );

        for c in elves[0].keys() {
            if elves[1].contains_key(c) && elves[2].contains_key(c) {
                total += to_priority(*c);
            }
        }
    }

    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() <= 1 {
        "./inputs/in3"
    } else {
        &args[1]
    };
    let file = File::open(file_path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let rucksacks: Vec<String> = lines
        .filter_map(|l| match l {
            Ok(s) if s.is_empty() => None,
            _ => l.ok(),
        })
        .collect();

    // let total = part1(rucksacks);
    let total = part2(rucksacks);

    println!("TOTAL = {}", total);
}

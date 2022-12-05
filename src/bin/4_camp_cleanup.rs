use std::collections::hash_map::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn part1(lines: Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines {
        let pair = &line.split(',').collect::<Vec<&str>>()[..];

        let fst = &pair[0]
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()[..];

        let snd = &pair[1]
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()[..];

        // dbg!("line = {} >> {}, {}", line, fst, snd);

        if (fst[0] <= snd[0] && fst[1] >= snd[1]) || (snd[0] <= fst[0] && snd[1] >= fst[1]) {
            total += 1
        }
    }

    total
}

fn part2(lines: Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines {
        let pair = &line.split(',').collect::<Vec<&str>>()[..];

        let fst = &pair[0]
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()[..];

        let snd = &pair[1]
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()[..];

        // dbg!("line = {} >> {}, {}", line, fst, snd);

        if !(fst[1] < snd[0] || fst[0] > snd[1]) {
            total += 1
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
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let total = part2(lines);

    println!("TOTAL = {}", total);
}

#[cfg(test)]
fn test_part1() {}

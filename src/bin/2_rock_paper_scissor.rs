use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Win,
    Draw,
    Lose,
}

impl GameResult {
    fn score(&self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPM {
    Rock,
    Paper,
    Scissors,
}

impl RPM {
    fn score(&self) -> i32 {
        match self {
            RPM::Rock => 1,
            RPM::Paper => 2,
            RPM::Scissors => 3,
        }
    }

    fn fight(&self, other: &RPM) -> GameResult {
        match (self, other) {
            (s, o) if s == o => GameResult::Draw,
            (RPM::Rock, RPM::Scissors) | (RPM::Paper, RPM::Rock) | (RPM::Scissors, RPM::Paper) => {
                GameResult::Win
            }
            _ => GameResult::Lose,
        }
    }

    fn will_win(&self) -> Self {
        match self {
            RPM::Rock => RPM::Scissors,
            RPM::Paper => RPM::Rock,
            RPM::Scissors => RPM::Paper,
        }
    }

    fn will_lose(&self) -> Self {
        match self {
            RPM::Rock => RPM::Paper,
            RPM::Paper => RPM::Scissors,
            RPM::Scissors => RPM::Rock,
        }
    }

    fn from_result(result: &GameResult, other: &RPM) -> Self {
        match (result, other) {
            (GameResult::Draw, o) => o.clone(),
            (GameResult::Win, o) => o.will_lose(),
            (GameResult::Lose, o) => o.will_win(),
        }
    }
}

fn abc_2_rpm(c: &str) -> RPM {
    match c {
        "A" => RPM::Rock,
        "B" => RPM::Paper,
        "C" => RPM::Scissors,
        _ => panic!("Expect ABC, gets {}", c),
    }
}

fn xyz_2_rpm(c: &str) -> RPM {
    match c {
        "X" => RPM::Rock,
        "Y" => RPM::Paper,
        "Z" => RPM::Scissors,
        _ => panic!("Expect XYZ, gets {}", c),
    }
}

fn xyz_2_rpm_result(c: &str) -> GameResult {
    match c {
        "X" => GameResult::Lose,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("Expect XYZ, gets {}", c),
    }
}

fn part_1(plans: &Vec<String>) -> i32 {
    let mut points: i32 = 0;
    for plan in plans {
        if plan == "" {
            println!("got empty plan");
            continue;
        }
        let splitted: Vec<&str> = plan.split(" ").collect();

        let op_rpm = abc_2_rpm(splitted[0]);
        let us_rpm = xyz_2_rpm(splitted[1]);

        points += us_rpm.score() + us_rpm.fight(&op_rpm).score();
    }

    points
}

fn part_2(plans: &Vec<String>) -> i32 {
    let mut points: i32 = 0;
    for plan in plans {
        if plan == "" {
            println!("got empty plan");
            continue;
        }

        let splitted: Vec<&str> = plan.split(" ").collect();

        let op_rpm = abc_2_rpm(splitted[0]);
        let game_result = xyz_2_rpm_result(splitted[1]);

        let us_rpm = RPM::from_result(&game_result, &op_rpm);

        // println!(
        //     "plan = {:?}, expect result {:?}, {:?} vs {:?}, score = {:?} + {:?}",
        //     &plan,
        //     game_result,
        //     op_rpm,
        //     us_rpm,
        //     us_rpm.score(),
        //     game_result.score()
        // );

        points += us_rpm.score() + game_result.score();
    }

    points
}

fn main() {
    let file_path = "./inputs/in2";
    let file = File::open(file_path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let plans: Vec<String> = lines
        .filter_map(|l| match l {
            Ok(s) if s.is_empty() => None,
            _ => l.ok(),
        })
        .collect();

    let points_1 = part_1(&plans);
    println!("Points part 1 = {}", points_1);

    let points_2 = part_2(&plans);

    println!("Points part 2 = {}", points_2);
}

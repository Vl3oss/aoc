use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "./inputs/in1";
    let file = File::open(file_path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut calories: Vec<u32> = vec![0];

    for line in lines {
        let maybe_num = line.unwrap();

        if maybe_num == "" {
            calories.push(0);
        } else {
            let current_cals = calories.last_mut().unwrap();
            *current_cals += maybe_num.parse::<u32>().unwrap();
        }
    }

    let mut heap = BinaryHeap::from(calories);
    println!("Sorted cals");

    let mut total = 0;
    for i in 0..3 {
        let cal = heap.pop().unwrap();
        total += cal;

        println!("{}. {}", i + 1, cal)
    }

    println!("TOTAL = {}", total);
}

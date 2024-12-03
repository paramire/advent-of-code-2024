use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn read_file(filepath: &str) -> String {
    let mut contents = String::new();
    let mut file = File::open(filepath).unwrap();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn mull_it_over_part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for (_, [val1, val2]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap();
    }
    sum
}

fn mull_it_over_part2(input: &str) -> i32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    let mut compute = true;
    for capture in re.captures_iter(input) {
        let rule = capture.get(0).unwrap().as_str();
        if rule == "don't()" {
            compute = false;
            continue;
        } else if rule == "do()" {
            compute = true;
            continue;
        }

        if compute {
            let val1 = capture.get(1).unwrap().as_str();
            let val2 = capture.get(2).unwrap().as_str();
            sum += val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap();
        }
    }
    sum
}

fn main() {
    let filepath = "./input.txt";
    let input_text = read_file(filepath);
    let total_part1 = mull_it_over_part1(&input_text);
    println!("Output: {}", total_part1);
    let total_part2 = mull_it_over_part2(&input_text);
    println!("Output:  {}", total_part2);
}

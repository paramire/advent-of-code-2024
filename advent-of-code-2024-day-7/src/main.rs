use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;
use std::iter::zip;

fn read_file(filepath: &str) -> String {
    let mut contents = String::new();
    let mut file = File::open(filepath).unwrap();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn process_data(contents: &str) -> Vec<(i64, Vec<i64>)>{
    contents.lines()
        .map(|line| {
            let (total_str, numbers) = line.split_once(":").unwrap();
            let total: i64 = total_str.parse().unwrap();
            let values: Vec<i64> = numbers.split_whitespace()
                .map(|v| { v.parse().unwrap() })
                .collect();
            (total, values)
        })
        .collect::<Vec<(i64, Vec<i64>)>>()
}


fn bridge_repair(operations: &Vec<(i64, Vec<i64>)>, operators: Vec<&str>) -> i64 {
    let mut success_total: i64 = 0;
    for (oper_total, oper_values) in operations {
        let num_opers = oper_values.len() - 1;
        for oper in std::iter::repeat(operators.clone()).take(num_opers).multi_cartesian_product() {
            let mut total = 0;
            for (index, _val) in oper_values.iter().enumerate() {
                if index == 0 {
                    total += oper_values[index];
                } else {
                    total = match oper[&index - 1] {
                        "*" => total * oper_values[index],
                        "+" => total + oper_values[index],
                        "||" => {
                            let value_length = oper_values[index].ilog10() + 1;
                            total * 10_i64.pow(value_length) + oper_values[index]
                        },
                        _other => total,
                    };
                }
                if total > *oper_total {
                    break;
                }
            }
            // Sum successes
            if total == *oper_total {
                success_total += total;
                break;
            }
        }
    }
    success_total
}

fn main() {
    let filepath = "./input.txt";
    let input_test = read_file(filepath);
    let operations = process_data(&input_test);
    let mut success_total: i64;
    success_total = bridge_repair(&operations, vec!["*", "+"]);
    println!("Part 1: {}", success_total);
    success_total = bridge_repair(&operations, vec!["*", "+", "||"]);
    println!("Part 2: {}", success_total);
}

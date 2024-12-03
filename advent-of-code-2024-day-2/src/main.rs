use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn read_file(filepath: &str) -> Vec<Vec<i32>> {
    let mut contents = String::new();
    let mut file = File::open(filepath).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let mut input_list = Vec::new();
    for line in contents.lines() {
        let values: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        input_list.push(values);
    }
    input_list
}

#[derive(PartialEq,Debug,Clone)]
enum Order {
    Ascending,
    Decreasing,
    Neither
}

#[derive(Debug,Clone)]
struct Rule {
    order: Order,
    check: bool
}

fn check_order(val1: i32, val2: i32) -> Order {
    if val1 > val2 {
        Order::Decreasing
    } else if val1 < val2 {
        Order::Ascending
    } else {
        Order::Neither
    }
}

fn check_rules(val1: i32, val2: i32) -> Rule {
    let check = (val1 - val2).abs() > 3 || (val1 - val2).abs() < 1;
    let order = check_order(val1, val2);
    Rule { check, order }
}

fn check_report(report: &Vec<i32>) -> bool {
    // Quick check, if the difference between first and last is more than
    // 3 * length of the report its always false
    let length: i32 = report.len().try_into().unwrap();
    if report[0] - report[report.len() - 1] > (length - 1) * 3i32 {
        return false
    }
    // Check inital order is Neither
    let order: Order = check_order(report[0], report[1]);
    match order {
        Order::Neither => return false,
        _ => ()
    }
    // Window check
    for item in report.windows(2) {
        // Chequear orden
        let rule: Rule = check_rules(item[0], item[1]);
        if rule.order != order {
            return false
        } else if rule.check {
            return false
        }
    }
    true
}

fn red_nosed_reports_part_1(report_lists: &Vec<Vec<i32>>) -> i32 {
    let mut successes: i32 = 0;
    for report in report_lists {
        let result = check_report(&report);
        if result {
            successes += 1;
        }
    }
    successes
}

fn red_nosed_reports_part_2(report_lists: &Vec<Vec<i32>>) -> i32 {
    let mut successes: i32 = 0;
    for report in report_lists {
        let mut result = check_report(&report);
        if result {
            successes += 1;
        } else {
            let mut chopped_report: Vec<i32>;
            for index in 0..report.len() {
                chopped_report = report.to_vec();
                chopped_report.remove(index);
                result = check_report(&chopped_report);
                if result {
                    successes += 1;
                    break;
                }
            }
        }
    }
    successes
}

fn main() {
    let filepath: &str = "./input.txt";
    let input_list: Vec<Vec<i32>> = read_file(filepath);

    let before = Instant::now();
    println!("Total successes: {}", red_nosed_reports_part_1(&input_list));
    println!("Elapsed Time {:.2?}", before.elapsed());

    println!("Total successes: {}", red_nosed_reports_part_2(&input_list));
    println!("Elapsed Time {:.2?}", before.elapsed());
}

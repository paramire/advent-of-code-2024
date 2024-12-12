use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_file(filepath: &str) -> String {
    let mut contents = String::new();
    let mut file = File::open(filepath).unwrap();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn process_data(contents: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let (str1, str2) = contents.split_once("\n\n").unwrap();
    let vec1: Vec<Vec<i32>> = str1.lines()
        .map(|x| {
            x.split('|')
            .map(|x| x.parse().unwrap())
            .collect()
        })
        .collect();
    let vec2: Vec<Vec<i32>> = str2.lines()
        .map(|x| {
            x.split(',')
            .map(|x| x.parse().unwrap())
            .collect()
        })
        .collect();
    (vec1, vec2)
}

fn build_page_order(order_info: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut page_order: HashMap<i32, Vec<i32>> = HashMap::new();
    for order in order_info {
        page_order.entry(order[1])
            .or_insert(vec![]);
        page_order.entry(order[0])
            .and_modify(|x| {
                x.push(order[1]);
            })
            .or_insert(vec![order[1]]);
    }
    page_order
}

fn check_updates(page_updates: &Vec<Vec<i32>>, page_orders: &HashMap<i32, Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut update_success: Vec<Vec<i32>> = vec![];
    let mut update_failed: Vec<Vec<i32>> = vec![];
    for update in page_updates {
        let mut success: bool = true;
        'outer: for (index, update_item) in update.iter().enumerate() {
            // For every element in update check forward if the elements next values are the elemnts
            // forward
            if index < update.len() {
                for forward_value in &update[index + 1..] {
                    if !page_orders[&update_item].contains(&forward_value) {
                        success = false;
                        break 'outer;
                    }
                }
            }
            // For every element in update check backward if the list of the backward_value contains
            // the current element
            if index > 0 {
                for backward_value in &update[..index] {
                    if page_orders[&update_item].contains(&backward_value) {
                        success = false;
                        break 'outer;
                    }
                }
            }
        }
        // Separate by result
        if success {
            update_success.push(update.clone());
        } else {
            update_failed.push(update.clone());
        }
    }
    (update_success, update_failed)
}

fn print_queue_part1(update_success: &Vec<Vec<i32>>) -> i32 {
    update_success
        .iter()
        .map(|upd| {
            upd[(upd.len() + 1) / 2 - 1]
        })
        .sum()
}

fn print_queue_part2(update_failed: &mut Vec<Vec<i32>>, page_orders: &HashMap<i32, Vec<i32>>) -> i32 {
    for update in &mut *update_failed {
        let mut index = 0;
        while index < update.len() {
            let mut is_changed = false;
            let current_item = update[index];
            // For every element forward check if they have the current value
            // if contains, we change the values and start over
            for index_val in (index + 1)..update.len() {
                let next_value = update[index_val];
                if page_orders[&next_value].contains(&current_item) {
                    update[index] = next_value;
                    update[index_val] = current_item;
                    is_changed = true;
                    break;
                }
            }
            // If hasnt changed move to the next item
            if !is_changed {
                index += 1;
            }
        }
    }

    update_failed
        .iter()
        .map(|upd| {
            upd[(upd.len() + 1) / 2 - 1]
        })
        .sum()
}

fn main() {
    let filepath: &str = "./input.txt";
    let input_data = read_file(filepath);
    let (order_info, update_info) = process_data(&input_data);
    let page_orders = build_page_order(&order_info);
    // Part 1
    let (succeed, mut failed) = check_updates(&update_info, &page_orders);
    println!("Total: {}", print_queue_part1(&succeed));
    // Part 2
    println!("Total: {}", print_queue_part2(&mut failed, &page_orders));
}

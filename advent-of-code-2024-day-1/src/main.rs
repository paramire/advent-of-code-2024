use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_file(filepath: &str) -> (Vec<i32>, Vec<i32>) {
    let mut contents = String::new();
    let mut file = File::open(filepath).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for line in contents.lines() {
        let values: Vec<i32> = line.split_whitespace()
            .take(2)
            .map(|x| x.parse().unwrap())
            .collect();
        vec1.push(values[0]);
        vec2.push(values[1]);
    }
    assert_eq!(vec1.len(), vec2.len(), "vec1 and vec 2 are not equal length");
    (vec1, vec2)
}

// Day 1
fn historian_hysteria_part_1(hist_list_1: &Vec<i32>, hist_list_2: &Vec<i32>) -> i32 {
    let mut vec1 = hist_list_1.clone();
    let mut vec2 = hist_list_2.clone();
    vec1.sort();
    vec2.sort();
    // Acumulative
    let mut accum: i32 = 0;
    for index in 0..vec1.len() {
        accum += (vec1[index] - vec2[index]).abs();
    }
    accum
}

fn historian_hysteria_part_2(hist_list_1: &Vec<i32>, hist_list_2: &Vec<i32>) -> i32 {
    let vec1 = hist_list_1.clone();
    let mut vec2 = hist_list_2.clone();
    vec2.sort();
    // Calculate Hashmap Counter
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for key in vec2 {
        counter.entry(key).and_modify(|x| *x += 1).or_insert(1);
    }
    // Calculate Similarity Score
    let mut score = 0;
    for val in vec1 {
        if let Some(x) = counter.get(&val) {
            score += val * x;
        }
    }
    score
}

fn main() {
    let filepath = "./input.txt";
    let hist_paths = read_file(filepath);
    let total_distance = historian_hysteria_part_1(&hist_paths.0, &hist_paths.1);
    println!("Total distance: {}", total_distance);
    let similarity_score = historian_hysteria_part_2(&hist_paths.0, &hist_paths.1);
    println!("similarity_score:  {}", similarity_score);
}

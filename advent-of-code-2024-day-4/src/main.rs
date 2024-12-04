use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn read_file(filepath: &str) -> Vec<Vec<char>> {
    let mut contents = String::new();
    let mut file = File::open(filepath).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let mut text_matrix: Vec<Vec<char>> = vec![];
    for line in contents.lines() {
        let values: Vec<char> = line.chars().collect();
        text_matrix.push(values)
    }
    text_matrix
}

fn count_xmas(xmas_line: &str) -> i32 {
    let re_xmas = Regex::new(r"XMAS").unwrap();
    let total_xmas: i32 = re_xmas.find_iter(xmas_line)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()
        .len()
        .try_into()
        .unwrap();
    let re_smax = Regex::new(r"SAMX").unwrap();
    let total_smax: i32 = re_smax.find_iter(xmas_line)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()
        .len()
        .try_into()
        .unwrap();
    total_xmas + total_smax
}

fn build_horizontal_lines(input: &Vec<Vec<char>>) -> Vec<String> {
    let mut new_lines: Vec<String> = vec![];
    for row in input {
        new_lines.push(row.into_iter().collect());
    }
    new_lines
}

fn build_vertical_lines(input: &Vec<Vec<char>>) -> Vec<String> {
    let max_length = input[0].len();
    let mut new_lines: Vec<String> = vec![String::new(); max_length];
    for row in input {
        for (index, c) in row.iter().enumerate() {
            new_lines[index].push(*c);
        }
    }
    new_lines
}

fn build_diagonal_lines(input: &Vec<Vec<char>>) -> Vec<String> {
    let max = input.len() - 1;
    let mut new_lines: Vec<String> = vec![String::new(); 2 * (max + 1) - 1];
    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    loop {
        // println!("x: {}, y: {}", x, y);
        // println!("max_x: {}; max_y: {}", max_x, max_y);
        new_lines[max_x + max_y].push(input[x][y]);
        if x == max_y && y == max_x {
            if max_x < max {
                max_x += 1;
            } else {
                max_y += 1;
            }
            x = max_x;
            y = max_y;
        } else {
            x -= 1;
            y += 1;
        }

        if max_y > max {
            break;
        }
    };
    new_lines
}

fn build_diagonal_lines_reversed(input: &Vec<Vec<char>>) -> Vec<String> {
    let mut rev_input: Vec<Vec<char>> = vec![];
    for row in input {
        let mut arr = row.clone();
        arr.reverse();
        rev_input.push(arr);
    }
    build_diagonal_lines(&rev_input)
}

fn ceres_search_part1(input: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    // Horizontal
    for line in build_horizontal_lines(input) {
        total += count_xmas(&line);
    }
    // Vertical
    for line in build_vertical_lines(input) {
        total += count_xmas(&line);
    }
    // Diagonal
    for line in build_diagonal_lines(input) {
        total += count_xmas(&line);
    }
    // Diagonal 2
    for line in build_diagonal_lines_reversed(input) {
        total += count_xmas(&line);
    }
    total
}

// Part 2
fn is_x_shaped(tr: char, tl: char, br: char, bl: char) -> bool {
    // If any X or A then drop
    if [tr, tl, br, bl].contains(&'X') || [tr, tl, br, bl].contains(&'A') {
        return false;
    }
    // Check shape
    if tr == tl && br == bl && tr != br {
        return true;
    } else if tr == br && tl == bl && tr != tl {
        return true;
    } else {
        return false;
    }
}

fn ceres_search_part2(input: &Vec<Vec<char>>) -> i32 {
    let mut total: i32 = 0;
    let length: usize = input.len();
    for i in 1..(length - 1) {
        for j in 1..(length - 1) {
            match input[i][j] {
                'A' => {
                    let is_mas = is_x_shaped(
                        input[i - 1][j - 1],
                        input[i - 1][j + 1],
                        input[i + 1][j - 1],
                        input[i + 1][j + 1]
                    );
                    if is_mas {
                        total += 1;
                    }
                }
                _ => (),
            }
        }
    }
    total
}

fn main() {
    let filepath = "./input.txt";
    let input_matrix = read_file(filepath);
    let total_part1 = ceres_search_part1(&input_matrix);
    println!("Output: {}", total_part1);
    let total_part2 = ceres_search_part2(&input_matrix);
    println!("Output: {}", total_part2);
}

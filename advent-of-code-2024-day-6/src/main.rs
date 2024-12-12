use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct HistoryPointDirection {
    x: i32,
    y: i32,
    d: String
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct HistoryPoint {
    x: i32,
    y: i32
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn read_file(filepath: &str) -> String {
    let mut contents = String::new();
    let mut file = File::open(filepath).unwrap();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn process_data(contents: &str) -> Vec<Vec<char>> {
    let mut data: Vec<Vec<char>> = vec![];
    for line in contents.lines() {
        data.push(line.chars().collect())
    }
    data
}

fn find_guard(data: &Vec<Vec<char>>) -> Option<Point> {
    for (row_index, row) in data.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == '^' {
                return Some(Point { x: col_index as i32, y: row_index as i32 });
            }
        }
    }
    None
}

fn get_next_direction(direction: &String) -> String {
    let directions = ["up", "right", "down", "left"];
    let index = directions.iter().position(|x| *x == direction).unwrap();
    if index + 1 <= directions.len() - 1 {
        String::from(directions[index + 1])
    } else {
        String::from(directions[0])
    }
}

fn update_guard(data: &Vec<Vec<char>>, x: i32, y: i32, direction: &mut String, guard: &mut Point) {
    let x = x as usize;
    let y = y as usize;
    if data[y][x] == '#' || data[y][x] == 'O' {
        *direction = get_next_direction(direction);
    } else {
        guard.x = x as i32;
        guard.y = y as i32;
    }
}

fn block_next<'a>(data: &mut Vec<Vec<char>>, history_point: &HistoryPointDirection, from_char: char, to_char: char) -> Result<HistoryPoint, &'a str> {
    let height = data.len() as i32;
    let width = data[0].len() as i32;
    let mut x = history_point.x as usize;
    let mut y = history_point.y as usize;
    match history_point.d.as_str() {
        "up" => {
            if history_point.y - 1 >= 0 {
                y -= 1;
            } else {
                return Err("Out of bounds");
            }
        }
        "down" => {
            if history_point.y + 1 < height {
                y += 1;
            } else {
                return Err("Out of bounds");
            }
        }
        "left" => {
            if history_point.x - 1 >= 0 {
                x -= 1;
            } else {
                return Err("Out of bounds");
            }
        }
        "right" => {
            if history_point.x + 1 < width {
                x += 1;
            } else {
                return Err("Out of bounds");
            }
        }
        _ => ()
    };
    if data[y][x] == from_char {
        data[y][x] = to_char;
    } else {
        return Err("Not a empty space");
    }
    Ok(HistoryPoint { x: x as i32, y: y as i32 })
}

fn traverse<'a>(data: &'a Vec<Vec<char>>, guard: &'a mut Point, direction: &'a mut String) -> Result<(HashSet<HistoryPoint>, HashSet<HistoryPointDirection>), &'a str> {
    let height = data.len() as i32;
    let width = data[0].len() as i32;
    let mut leave = false;
    let mut history_points: HashSet<HistoryPoint> = HashSet::new();
    let mut history_direction_points: HashSet<HistoryPointDirection> = HashSet::new();
    loop {
        history_points.insert(HistoryPoint { x: guard.x, y: guard.y });
        let history_point = HistoryPointDirection { x: guard.x, y: guard.y, d: direction.clone() };
        if !history_direction_points.contains(&history_point) {
            history_direction_points.insert(history_point);
        } else {
            return Err("Loop")
        }

        // Move
        match direction.as_str() {
            "up" => {
                if guard.y - 1 >= 0{
                    update_guard(&data, guard.x, guard.y - 1, direction, guard);
                } else {
                    leave = true;
                }
            }
            "down" => {
                if guard.y + 1 < height {
                    update_guard(&data, guard.x, guard.y + 1, direction, guard);
                } else {
                    leave = true;
                }
            }
            "left" => {
                if guard.x - 1 >= 0 {
                    update_guard(&data, guard.x - 1, guard.y, direction, guard);
                } else {
                    leave = true;
                }
            }
            "right" => {
                if guard.x + 1 < width {
                    update_guard(&data, guard.x + 1, guard.y, direction, guard);
                } else {
                    leave = true;
                }
            }
            _ => break
        }

        if leave {
            break
        }
    }
    return Ok((history_points, history_direction_points))
}

fn main() {
    let filepath: &str = "./input_real.txt";
    let input_data = read_file(filepath);
    let mut data = process_data(&input_data);
    // Guard status
    let mut direction = String::from("up");
    let mut guard = find_guard(&data).unwrap();
    let start = HistoryPoint { x: guard.x, y: guard.y };
    // Part 1
    let history = traverse(&data, &mut guard, &mut direction).unwrap();
    println!("{:?}", history.0.len());
    // println!("{:?}", history.1.len());

    // Part 2
    let mut blockage_points: HashSet<HistoryPoint> = HashSet::new();
    for hist_point in history.1 {
        if start == (HistoryPoint { x: hist_point. x, y: hist_point.y }) {
            continue;
        }
        let block_point = block_next(&mut data, &hist_point, '.', 'O');
        match block_point {
            Err(_) => continue,
            Ok(_) => (),
        }
        // println!("{:?}", hist_point);
        let mut new_guard = find_guard(&data).unwrap();
        let mut new_direction = String::from("up");
        if let Err(_) = traverse(&data, &mut new_guard, &mut new_direction) {
            blockage_points.insert(block_point.unwrap());
        }
        block_next(&mut data, &hist_point, 'O', '.');
    }
    println!("{:?}", blockage_points.len());
}

use std::fs;

fn read_file_to_lines(file_path: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn check_direction(x: i32, y: i32, delta_x: i32, delta_y: i32, map: &Vec<Vec<String>>) -> bool {
    let mut i = x;
    let mut j = y;
    for char in ["M", "A", "S"] {
        i += delta_x;
        j += delta_y;
        if i < 0 || j < 0 || i >= map.len() as i32 || j >= map[0].len() as i32 {
            return false;
        }
        if char != map[i as usize][j as usize] {
            return false;
        }
    }

    true
}

fn part_one(file_name: &str) -> i32 {
    let lines = read_file_to_lines(file_name);
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char != "X" {
                continue;
            }
            for delta_i in -1..2 {
                for delta_j in -1..2 {
                    if check_direction(i as i32, j as i32, delta_i, delta_j, &lines) {
                        sum += 1;
                    }
                }
            }
        }
    }

    sum
}

fn check_cross(x: i32, y: i32, map: &Vec<Vec<String>>) -> bool {
    if map[x as usize][y as usize] != "A" {
        return false;
    }
    if x < 1 || y < 1 || x >= map.len() as i32 - 1  || y >= map[0].len() as i32 - 1 {
        return false;
    }
    let top_left = map[(x - 1) as usize][(y - 1) as usize].as_str();
    let top_right = map[(x - 1) as usize][(y + 1) as usize].as_str();
    let bottom_left = map[(x + 1) as usize][(y - 1) as usize].as_str();
    let bottom_right = map[(x + 1) as usize][(y + 1) as usize].as_str();
    let first_diag = match top_left {
        "M" => { bottom_right == "S" }
        "S" => { bottom_right == "M" }
        _ => { false }
    };
    let second_diag = match top_right {
        "M" => { bottom_left == "S" }
        "S" => { bottom_left == "M" }
        _ => { false }
    };

    first_diag && second_diag
}

fn part_two(file_name: &str) -> i32 {
    let lines = read_file_to_lines(file_name);
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if check_cross(i as i32, j as i32, &lines) {
                sum += 1;
            }
        }
    }
    sum
}

fn main() {
    println!("Part 1 test: {}", part_one("test.txt"));
    println!("Part 1: {}", part_one("input.txt"));
    println!("Part 2 test: {}", part_two("test.txt"));
    println!("Part 2: {}", part_two("input.txt"));
}

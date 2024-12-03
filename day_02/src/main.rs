use std::fs;

fn read_file_to_lines(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents.split("\n").map(|s| s.to_string()).filter(|s| s.len() > 0).collect()
}

fn format_data (data: &Vec<String>) -> Vec<Vec<i32>> {
    data
        .iter()
        .map(|line| line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
        ).collect()
}

fn find_wrong_reports(data: &Vec<Vec<i32>>) -> usize {
    data.iter().filter(|line| {
        let mut is_rising: Option<bool> = None;
        for (idx, val) in line.iter().enumerate() {
            if idx < 1 {
                continue;
            }
            let prev_val = line[idx - 1];
            if is_rising.is_none() {
                is_rising = Some(prev_val < *val);
            }
            if !check_two_levels(prev_val, *val, is_rising.unwrap()) {
                return false;
            }
        }
        true
    }).count()
}

fn check_two_levels (prev: i32, curr: i32, is_rising: bool) -> bool {
    if is_rising && prev > curr {
        return false;
    }
    if !is_rising && prev < curr {
        return false;
    }
    let dif = curr - prev;
    if dif.abs() < 1 || dif.abs() > 3 {
        return false;
    }
    true
}

fn check_line(line: &Vec<i32>) -> bool {
    let is_rising = line[0] < line[1];
    for (idx, val) in line.iter().enumerate() {
        if idx < 1 {
            continue;
        }
        let prev_val = line[idx - 1];
        if !check_two_levels(prev_val, *val, is_rising) {
            return false;
        }
    }
    true
}

fn find_wrong_reports_with_dampener(data: &Vec<Vec<i32>>) -> usize {
    data.iter().filter(|line| {
        if check_line(line) {
            return true;
        }
        for (idx, _) in line.iter().enumerate() {
            let mut copy_line = line.clone().clone();
            copy_line.remove(idx);
            if check_line(&copy_line) {
                return true;
            }
        }
        false
    }).count()
}

fn part_one(file_path: &str) -> usize {
    let lines = read_file_to_lines(file_path);
    let formated = format_data(&lines);
    let result =  find_wrong_reports(&formated);
    result
}

fn part_two(file_path: &str) -> usize {
    let lines = read_file_to_lines(file_path);
    let formated = format_data(&lines);
    let result =  find_wrong_reports_with_dampener(&formated);
    result
}

fn main() {
    println!("Part One test: {}", part_one("test_1.txt"));
    println!("Part One: {}", part_one("input.txt"));
    println!("Part Two test: {}", part_two("test_1.txt"));
    println!("Part Two: {}", part_two("input.txt"));
}

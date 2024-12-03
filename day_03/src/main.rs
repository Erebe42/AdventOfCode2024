use std::fs;
use regex::Regex;

fn read_file_to_lines(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}

fn part_one(file_name: &str) -> i32 {
    let content = read_file_to_lines(file_name);
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for capture in regex.captures_iter(&content) {
        let first = capture[1].parse::<i32>().unwrap();
        let second = capture[2].parse::<i32>().unwrap();
        sum += first * second;
    }

    sum
}

fn part_two(file_name: &str) -> i32 {
    let content = read_file_to_lines(file_name);
    let regex = Regex::new(r"(?:(mul)\((\d{1,3}),(\d{1,3})\))|(?:(do)\(\))|(?:(don't)\(\))").unwrap();
    let mut sum = 0;
    let mut do_mul = true;
    for capture in regex.captures_iter(&content) {
        if capture[0].contains("mul") {
            if do_mul {
                let first = capture[2].parse::<i32>().unwrap();
                let second = capture[3].parse::<i32>().unwrap();
                sum += first * second;
            }
        }
        else if capture[0].contains("don't") {
            do_mul = false;
        }
        else if capture[0].contains("do") {
            do_mul = true;
        }
    }

    sum
}

fn main() {
    println!("Part One test: {}", part_one("test_1.txt"));
    println!("Part One: {}", part_one("input.txt"));
    println!("Part Two test: {}", part_two("test_2.txt"));
    println!("Part Two: {}", part_two("input.txt"));
}

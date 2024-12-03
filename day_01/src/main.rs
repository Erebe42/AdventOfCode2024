use std::fs;

fn read_file_to_lines(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents.split("\n").map(|s| s.to_string()).collect()
}

fn get_left_right(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        if line.len() < 2 { continue; }
        let number = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        left.push(number[0]);
        right.push(number[1]);
    }

    (left, right)
}

fn calculate_total_dist(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    let mut sum = 0;
    left.sort();
    right.sort();

    for (i, left_value) in left.iter().enumerate() {
        let right_value = right[i];
        let dist = right_value - left_value;
        sum += dist.abs();
    }

    sum
}

fn calculate_total_similarity(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    let mut sum = 0;
    left.sort();
    right.sort();

    for left_value in left {
        let right_occurence = right
            .iter()
            .filter(|x| **x == left_value)
            .collect::<Vec<&i32>>()
            .len() as i32;
        let dist = left_value * right_occurence;
        sum += dist.abs();
    }

    sum
}

fn part_one(file_name: &str) -> i32 {
    let lines = read_file_to_lines(file_name);
    let (left, right) = get_left_right(lines);
    calculate_total_dist(left, right)
}

fn part_two(file_name: &str) -> i32 {
    let lines = read_file_to_lines(file_name);
    let (left, right) = get_left_right(lines);
    calculate_total_similarity(left, right)
}

fn main() {
    println!("Part One test: {}", part_one("test_1.txt"));
    println!("Part One: {}", part_one("input.txt"));
    println!("Part One test: {}", part_two("test_1.txt"));
    println!("Part One: {}", part_two("input.txt"));
}


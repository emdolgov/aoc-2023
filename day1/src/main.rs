
use std::{fs::read_to_string, collections::HashMap};

use regex::Regex;

fn main() {
    let lines = read_lines("./src/input.txt".to_owned());
    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<String>) {
    let part_one_sum : i32 = lines.iter()
        .map(|x| get_line_sum(x.to_string()))
        .sum();
    println!("part 1 answer: {:?}", part_one_sum);
}

fn part_two(lines: &Vec<String>){
    let part_two_sum : i32 = lines.iter()
        .map(|x| get_line_sum_with_regex(x.to_string()))
        .sum();
    println!("part 2 answer: {:?}", part_two_sum);

}

fn read_lines(path: String) -> Vec<String> {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn get_line_sum(line: String) -> i32 {
    let numeric_digits : Vec<i32> = line
        .chars()
        .into_iter()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();

    return match numeric_digits.len()  {
        1 => numeric_digits[0] * 10 + numeric_digits[0],
        2 => numeric_digits[0] * 10 + numeric_digits[1],
        _ if numeric_digits.len() > 2 => numeric_digits[0]*10 + numeric_digits.last().unwrap(),
        _ => 0 
    }
}

fn get_line_sum_with_regex(line: String) -> i32 {
    //probably the least effective way to do this, but after spending half of the day setting up something I didn't end up using I couldn't be bothered to think :)
    let pattern = Regex::new("one|two|three|four|five|six|seven|eight|nine|zero|\\d").unwrap();

    let mut map = HashMap::new();
    map.insert("one".to_owned(), 1);
    map.insert("two".to_owned(), 2);
    map.insert("three".to_owned(), 3);
    map.insert("four".to_owned(), 4);
    map.insert("five".to_owned(), 5);
    map.insert("six".to_owned(), 6);
    map.insert("seven".to_owned(), 7);
    map.insert("eight".to_owned(), 8);
    map.insert("nine".to_owned(), 9);

    
    let mut matches: Vec<String> = Vec::new();
    let mut start = 0;

    while let Some(mat) = pattern.find_at(&line, start) {
       matches.push(mat.as_str().to_string());
       start = mat.start() + 1;
    }

    let (first, second) : (String, String) = match matches.len() {
        1 => (matches[0].clone(), matches[0].clone()),
        2 => (matches[0].clone(), matches[1].clone()),
        _ if matches.len() > 2 => (matches[0].clone(), matches[matches.len() - 1].clone()),
        _ => (String::new(), String::new())
    };

    let first_num = match first.len() {
        1 => first.chars().next().unwrap().to_digit(10).unwrap_or(0) as i32,
        _ if first.len() > 1 => map.get(&first).cloned().unwrap_or(0) as i32,
        _ => 0
    };

    let second_num = match second.len() {
        1 => second.chars().next().unwrap().to_digit(10).unwrap_or(0) as i32,
        _ if second.len() > 1 => map.get(&second).cloned().unwrap_or(0) as i32,
        _ => 0
    };

    return first_num * 10 + second_num;
}
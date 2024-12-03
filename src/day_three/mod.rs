use std::fs;

use regex::Regex;

pub fn part_one() {
    let content = fs::read_to_string("src/day_three/input.txt").unwrap();

    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Regex error.");

    let result: i32 = reg
        .captures_iter(&content)
        .map(|capture| {
            let one = capture[1].parse::<i32>().unwrap();
            let two = capture[2].parse::<i32>().unwrap();

            one * two
        })
        .sum();

    println!("* Part one: {}", result);
}

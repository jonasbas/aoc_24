use std::fs;

fn main() {
    let content = fs::read_to_string("src/input.txt").unwrap();
    let lines = content.lines();

    let mut list_one = vec![];
    let mut list_two = vec![];

    lines.for_each(|line| {
        let tmp: Vec<&str> = line.split("   ").collect();

        let value_one = tmp[0].parse::<i32>().unwrap();
        let value_two = tmp[1].parse::<i32>().unwrap();

        list_one.push(value_one);
        list_two.push(value_two);
    });

    list_one.sort();
    list_two.sort();

    let result: i32 = list_one
        .iter()
        .zip(list_two)
        .map(|(x, y)| i32::abs(x - y))
        .sum();

    println!("{}", result);
}

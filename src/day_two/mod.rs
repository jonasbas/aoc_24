use std::fs;

pub fn part_one() {
    let content = fs::read_to_string("src/day_two/input.txt").unwrap();
    let lines = content.lines();

    let result: i32 = lines
        .map(|line| {
            let values: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
            let mut is_increasing = false;

            let is_safe = values.windows(2).enumerate().all(|(idx, x)| {
                let first = x[0];
                let second = x[1];

                // check for initial direction
                if idx == 0 {
                    is_increasing = first > second;
                }

                // check if always a- or descending
                let difference = first - second;
                if difference < 0 && is_increasing {
                    return false;
                }

                if difference > 0 && !is_increasing {
                    return false;
                }

                // check if difference is to big or small
                let difference_abs = i32::abs(difference);
                if difference_abs > 3 || difference_abs < 1 {
                    return false;
                }

                true
            });

            if is_safe {
                return 1;
            }

            0
        })
        .sum();

    println!("* Part one: {}", result);
}

use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

// Today is evil functional programming day.

fn input_iter() -> impl Iterator<Item = Box<[u8]>> {
    BufReader::new(File::open("data/day_2.txt").unwrap())
        .lines()
        .map(|v| v.unwrap().split(' ').map(|v| v.parse::<u8>().unwrap()).collect::<Box<[_]>>())
}

fn check_sorting(array: &[u8]) -> bool {
    // Check if the maximum difference between two consecutive elements is within 1..=3.
    array.is_sorted() || array.iter().rev().is_sorted()
}

fn check_levels(array: &[u8]) -> bool {
    // Check if the maximum difference between two consecutive elements is within 1..=3.
    array.windows(2).map(|v| v[0].abs_diff(v[1])).all(|v| v > 0 && v <= 3)
}

pub fn part_1() -> Result<()> {
    println!("{}", self::input_iter().filter(|v| check_sorting(v) && check_levels(v)).count());

    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut correct = 0;

    for array in self::input_iter() {
        if self::check_sorting(&array) && self::check_levels(&array) {
            correct += 1;

            continue;
        }

        // Dumb brute force
        for index in 0 .. array.len() {
            let mut temp_array = array.to_vec();

            temp_array.remove(index);

            if self::check_sorting(&temp_array) && self::check_levels(&temp_array) {
                correct += 1;

                break;
            }
        }
    }

    println!("{correct}");

    Ok(())
}

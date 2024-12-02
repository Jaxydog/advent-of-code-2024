use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

#[allow(clippy::type_complexity)]
fn parse_input() -> Result<(Box<[u32]>, Box<[u32]>)> {
    let file = File::open("data/day_1.txt")?;
    let iterator = BufReader::new(file).lines();

    let capacity = iterator.size_hint().1.unwrap_or_else(|| iterator.size_hint().0);
    let mut lhs_array = Vec::<u32>::with_capacity(capacity);
    let mut rhs_array = Vec::<u32>::with_capacity(capacity);

    for result in iterator {
        let line = result?;
        let (lhs_str, rhs_str) = line.split_once("   ").unwrap();

        lhs_array.push(lhs_str.parse()?);
        rhs_array.push(rhs_str.parse()?);
    }

    Ok((lhs_array.into_boxed_slice(), rhs_array.into_boxed_slice()))
}

pub fn part_1() -> Result<()> {
    let (mut lhs_array, mut rhs_array) = self::parse_input()?;
    let mut differences = Vec::with_capacity(lhs_array.len());

    lhs_array.sort_unstable();
    rhs_array.sort_unstable();

    for (index, lhs_value) in lhs_array.into_iter().enumerate() {
        let rhs_value = rhs_array[index];
        let difference = lhs_value.abs_diff(rhs_value);

        differences.push(difference as u64);
    }

    println!("{}", differences.into_iter().sum::<u64>());

    Ok(())
}

pub fn part_2() -> Result<()> {
    let (lhs_array, rhs_array) = self::parse_input()?;
    let mut multiples = Vec::with_capacity(lhs_array.len());

    for lhs_value in lhs_array {
        let rhs_appearances = rhs_array.iter().filter(|v| **v == lhs_value).count();

        multiples.push(lhs_value as u64 * rhs_appearances as u64);
    }

    println!("{}", multiples.into_iter().sum::<u64>());

    Ok(())
}

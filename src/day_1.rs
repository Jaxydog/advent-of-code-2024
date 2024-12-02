use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;

use crate::SolutionResult;

type Input = (Box<[u32]>, Box<[u32]>);

fn input(path: impl AsRef<Path>) -> Result<Input> {
    let iterator = BufReader::new(File::open(path)?).lines();
    let capacity = iterator.size_hint().1.unwrap_or_else(|| iterator.size_hint().0);

    let mut lhs_array = Vec::<u32>::with_capacity(capacity);
    let mut rhs_array = Vec::<u32>::with_capacity(capacity);

    // Split and parse every line into two separate numbers.
    for result in iterator {
        let line = result?;
        let (lhs_str, rhs_str) = line.split_once("   ").unwrap();
        //            This is exactly three spaces ^

        lhs_array.push(lhs_str.parse()?);
        rhs_array.push(rhs_str.parse()?);
    }

    Ok((lhs_array.into_boxed_slice(), rhs_array.into_boxed_slice()))
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    let (mut lhs_array, mut rhs_array) = self::input(path)?;
    let mut differences = Vec::with_capacity(lhs_array.len());

    // Sorting these automatically fills the requirement of matching lesser values together.
    lhs_array.sort_unstable();
    rhs_array.sort_unstable();

    for (index, lhs_value) in lhs_array.into_iter().enumerate() {
        // This index is fine because both arrays are always the same length.
        let difference = lhs_value.abs_diff(rhs_array[index]);

        differences.push(difference as u64);
    }

    Ok(differences.into_iter().sum())
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    let (lhs_array, rhs_array) = self::input(path)?;
    let mut multiples = Vec::with_capacity(lhs_array.len());

    for lhs_value in lhs_array {
        let rhs_appearances = rhs_array.iter().filter(|v| **v == lhs_value).count();

        // Make sure we cast beforehand so that we don't run into int size limitations.
        multiples.push(lhs_value as u64 * rhs_appearances as u64);
    }

    Ok(multiples.into_iter().sum())
}

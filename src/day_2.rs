use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;

use crate::SolutionResult;

type Input = <Box<[Box<[u8]>]> as IntoIterator>::IntoIter;

fn input(path: impl AsRef<Path>) -> Result<Input> {
    let iterator = BufReader::new(File::open(path)?).lines();
    let capacity = iterator.size_hint().1.unwrap_or_else(|| iterator.size_hint().0);

    let mut report_array = Vec::with_capacity(capacity);

    for result in iterator {
        let line = result?;

        let iterator = line.split(' ');
        let capacity = iterator.size_hint().1.unwrap_or_else(|| iterator.size_hint().0);

        let mut value_array = Vec::with_capacity(capacity);

        for substring in line.split(' ') {
            value_array.push(substring.parse()?);
        }

        report_array.push(value_array.into_boxed_slice());
    }

    Ok(report_array.into_iter())
}

fn check_sorting(array: &[u8]) -> bool {
    // Check if the array is sorted either forwards *or* backwards.
    array.is_sorted() || array.iter().rev().is_sorted()
}

fn check_levels(array: &[u8]) -> bool {
    // Check if the maximum difference between two consecutive elements is within 1..=3.
    array.windows(2).map(|v| v[0].abs_diff(v[1])).all(|v| v > 0 && v <= 3)
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    Ok(self::input(path)?.filter(|v| check_sorting(v) && check_levels(v)).count() as _)
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    let mut correct = 0;

    for report_array in self::input(path)? {
        if self::check_sorting(&report_array) && self::check_levels(&report_array) {
            correct += 1;

            continue;
        }

        let mut buffer = Vec::with_capacity(report_array.len().saturating_sub(1));

        // Dumb brute force check for removing single items.
        // At least we re-use the buffer!
        for index in 0 .. report_array.len() {
            buffer.clear();
            buffer.extend(report_array.iter().copied().enumerate().filter_map(|(i, n)| (i != index).then_some(n)));

            if self::check_sorting(&buffer) && self::check_levels(&buffer) {
                correct += 1;

                break;
            }
        }
    }

    Ok(correct)
}

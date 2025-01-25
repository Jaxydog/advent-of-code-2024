use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;

use crate::SolutionResult;

type Input = Box<[(u64, Box<[u64]>)]>;

fn input(path: impl AsRef<Path>) -> Result<Input> {
    let iterator = BufReader::new(File::open(path)?).lines();
    let capacity = iterator.size_hint().1.unwrap_or_else(|| iterator.size_hint().0);
    let mut list = Vec::with_capacity(capacity);

    for result in iterator {
        let line = result?;
        let (target, values) = line.split_once(": ").unwrap();

        let target = target.parse()?;
        // I love that you can collect into a result of a collection.
        let values = values.split(" ").map(|v| v.parse()).collect::<Result<_, _>>()?;

        list.push((target, values));
    }

    Ok(list.into_boxed_slice())
}

// Awful, awful recursive function. But it works!
fn find(target: u64, current: u64, list: &[u64], index: usize, ops: &[Box<dyn Fn(u64, u64) -> u64>]) -> Option<u64> {
    if target == current {
        return Some(target);
    } else if index >= list.len() || target < current {
        // If we run out of numbers, or miss the target.
        return None;
    }

    // We basically call every given operator recursively until we find the target value.
    // This *does* break out early if the value is found, so it could be worse!
    ops.iter().find_map(|f| self::find(target, f(current, list[index]), list, index + 1, ops))
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    let mut sum = 0;

    for (target, values) in self::input(path)? {
        self::find(target, values[0], &values, 1, &[Box::from(|a, b| a + b), Box::from(|a, b| a * b)])
            .inspect(|v| sum += v);
    }

    Ok(sum as _)
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    let mut sum = 0;

    for (target, values) in self::input(path)? {
        self::find(target, values[0], &values, 1, &[
            Box::from(|a, b| a + b),
            Box::from(|a, b| a * b),
            // This works. It sucks, but it works.
            Box::from(|a, b| format!("{a}{b}").parse().unwrap()),
        ])
        .inspect(|v| sum += v);
    }

    Ok(sum as _)
}

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Result, bail};

use crate::SolutionResult;

type Input = Manual;

// `u8` should be enough to hold any number in our input.
struct Manual {
    rules: Box<[(u8, u8)]>,
    updates: Box<[Box<[u8]>]>,
}

struct ManualIndex<'m> {
    inner: &'m Manual,
    // Keeps track of every number and the values that are allowed to be below or above it.
    cache: HashMap<u8, (HashSet<u8>, HashSet<u8>)>,
}

impl<'m> ManualIndex<'m> {
    pub fn new(manual: &'m Manual) -> Self {
        Self { inner: manual, cache: HashMap::new() }
    }

    pub fn setup(&mut self) {
        for (lhs, rhs) in self.inner.rules.iter().copied() {
            self.cache.entry(lhs).or_default().1.insert(rhs);
            self.cache.entry(rhs).or_default().0.insert(lhs);
        }
    }

    /// Sorts two values according to the inner manual's paging rules.
    pub fn sort(&self, lhs: &u8, rhs: &u8) -> Option<Ordering> {
        assert!(!self.cache.is_empty(), "make sure you run `.setup` stupid");

        let (lesser, greater) = self.cache.get(lhs)?;

        Some(if lesser.contains(rhs) {
            Ordering::Greater
        } else if greater.contains(rhs) {
            Ordering::Less
        } else {
            Ordering::Equal
        })
    }

    /// Returns an iterator over the already sorted updates.
    pub fn sorted(&self) -> impl Iterator<Item = &[u8]> {
        self.inner.updates.iter().filter_map(|v| {
            // We want the arrays to be sorted, ascending, in accordance with the manual's rules.
            v.is_sorted_by(|a, b| self.sort(a, b).is_some_and(|v| v.is_le())).then_some(&**v)
        })
    }

    /// Returns an iterator that actively sorts the updates.
    pub fn sorting(&self) -> impl Iterator<Item = Box<[u8]>> {
        self.inner.updates.iter().map(|v| {
            let mut array = Box::<[u8]>::from(&**v);

            // We can safely assume any unexpected values should be treated as equals.
            // Y'know, we could learn a lot from these bytes.
            array.sort_unstable_by(|a, b| self.sort(a, b).unwrap_or(Ordering::Equal));

            array
        })
    }
}

fn input(path: impl AsRef<Path>) -> Result<Input> {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    // Track whether we've met the separating line.
    let mut finished_rules = false;

    for result in BufReader::new(File::open(path)?).lines() {
        let line = result?;

        if line.is_empty() {
            finished_rules = true;

            continue;
        }

        if finished_rules {
            let mut list = Vec::new();

            for number in line.split(',') {
                list.push(number.parse()?);
            }

            updates.push(list.into_boxed_slice());
        } else {
            let Some((lhs, rhs)) = line.split_once('|') else {
                bail!("invalid rule format");
            };

            rules.push((lhs.parse()?, rhs.parse()?));
        }
    }

    Ok(Manual { rules: rules.into_boxed_slice(), updates: updates.into_boxed_slice() })
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    let manual = self::input(path)?;
    let mut index = ManualIndex::new(&manual);

    index.setup();

    Ok(index
        .sorted()
        .map(|v| {
            // Grab the middle-most value and up-cast it.
            v[v.len() / 2] as u64
        })
        .sum::<u64>() as _)
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    let manual = self::input(path)?;
    let mut index = ManualIndex::new(&manual);

    index.setup();

    Ok(index
        .sorting()
        .filter(|v| {
            // Only allow values that have not yet been sorted.
            !index.sorted().any(|v2| &**v == v2)
        })
        .map(|v| {
            // Grab the middle-most value and up-cast it.
            v[v.len() / 2] as u64
        })
        .sum::<u64>() as _)
}

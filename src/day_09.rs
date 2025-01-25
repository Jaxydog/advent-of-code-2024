use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{Result, bail};

use crate::SolutionResult;

type Input = Box<[Block]>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Named(u16, u8),
    Empty(u8),
}

impl Block {
    /// Returns the id that should be used to represent this block.
    pub const fn id(&self) -> Option<u16> {
        match self {
            Block::Named(digit, _) => Some(*digit),
            Block::Empty(_) => None,
        }
    }

    /// Returns the length taken up by this block.
    pub const fn len(&self) -> usize {
        match self {
            Block::Named(_, len) | Block::Empty(len) => *len as usize,
        }
    }
}

fn input(path: impl AsRef<Path>) -> Result<Input> {
    let mut iterator = File::open(path)?.bytes();
    let (min_capacity, max_capacity) = iterator.size_hint();
    let capacity = max_capacity.unwrap_or(min_capacity);

    let mut list = Vec::with_capacity(capacity);
    let mut next_id = 0;
    let mut use_empty = false;

    while let Some(byte) = iterator.next().transpose()? {
        let character = char::from(byte);

        if let Some(digit) = character.to_digit(10).map(|v| v as u8) {
            list.push(if use_empty { Block::Empty(digit) } else { Block::Named(next_id, digit) });

            use_empty = !use_empty;
            next_id += use_empty as u16;
        } else if character != '\n' {
            bail!("invalid digit: {character:?}");
        }
    }

    Ok(list.into_boxed_slice())
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    let blocks = self::input(path)?;

    let disk: Box<[_]> = blocks.iter().flat_map(|v| std::iter::repeat(v.id()).take(v.len())).collect();
    let disk = RefCell::new(disk);

    let next_none_index = std::iter::from_fn(|| disk.borrow().iter().enumerate().find(|v| v.1.is_none()).map(|v| v.0));
    let last_some_index = std::iter::from_fn(|| disk.borrow().iter().enumerate().rfind(|v| v.1.is_some()).map(|v| v.0));
    let mut iterator = next_none_index.zip(last_some_index);

    while let Some((none_index, some_index)) = iterator.next().filter(|(a, b)| a < b) {
        disk.borrow_mut().swap(none_index, some_index);
    }

    Ok(disk.borrow().iter().enumerate().map(|(i, v)| v.map_or(0, |v| v as u64) * i as u64).sum())
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    todo!("solution not yet implemented for {}", path.as_ref().to_string_lossy())
}

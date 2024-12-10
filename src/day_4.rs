use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::{Result, bail};

use crate::SolutionResult;
use crate::utility::grid::{Grid2D, Offset2D, Pos2D, Size2D};

// Split the input string into a grid of characters.
type Input = Grid2D<char>;

fn input(path: impl AsRef<Path>) -> Result<Input> {
    let string = read_to_string(path)?;

    let w = string.lines().next().map_or(0, |v| v.len());
    let h = string.lines().count();
    let Some(size) = Size2D::try_new(w, h) else { bail!("invalid grid size") };

    let mut grid = Grid2D::new(size);

    for (y, line) in string.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            grid.set(Pos2D::new(x, y), character);
        }
    }

    Ok(grid)
}

// Recursive search algorithm to look for characters in a given direction until the stack is empty or the position
// goes out of bounds.
fn search_direction(grid: &Grid2D<char>, pos: Pos2D, offset: Offset2D, stack: &[char], index: usize) -> bool {
    stack.get(index).is_none_or(|c| {
        let Some(pos) = pos.offset(offset) else { return false };

        grid.get(pos).is_some_and(|v| v == c) && search_direction(grid, pos, offset, stack, index + 1)
    })
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    const PATTERN: &[char] = &['X', 'M', 'A', 'S'];

    let grid = self::input(path)?;
    let mut count = 0;

    for (pos, option) in grid.iter() {
        let Some(character) = option else { continue };

        // We need to manually check the first character, since we don't know the direction yet.
        if PATTERN.first().is_some_and(|v| v == character) {
            // Check in every direction from the target position.
            count += Offset2D::directions().filter(|d| self::search_direction(&grid, pos, *d, PATTERN, 1)).count();
        }
    }

    Ok(count as _)
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    const PATTERN: &[char] = &['M', 'A', 'S'];

    let grid = self::input(path)?;
    let mut centerpoints = BTreeMap::<Pos2D, usize>::new();

    for (pos, option) in grid.iter() {
        let Some(character) = option else { continue };

        // We need to manually check the first character, since we don't know the direction yet.
        if PATTERN.first().is_none_or(|v| v != character) {
            continue;
        }

        for offset in Offset2D::directions().filter(|d| {
            // Only allow diagonal directions, and only run the body for successful searches.
            (d.x() != 0 && d.y() != 0) && self::search_direction(&grid, pos, *d, PATTERN, 1)
        }) {
            let Some(pos) = pos.offset(offset) else { continue };

            *centerpoints.entry(pos).or_default() += 1;
        }
    }

    Ok(centerpoints.into_values().filter(|v| *v > 1).count() as _)
}

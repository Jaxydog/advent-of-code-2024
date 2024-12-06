use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

use crate::SolutionResult;

// Split the input string into a grid of characters.
type Input = Box<[Box<[char]>]>;

/// A side on the X axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SideX {
    Left,
    Right,
}

impl SideX {
    pub const fn offset(self) -> isize {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

// A side on the Y axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SideY {
    Up,
    Down,
}

impl SideY {
    pub const fn offset(self) -> isize {
        match self {
            Self::Up => -1,
            Self::Down => 1,
        }
    }
}

// A combination of an X and Y side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Direction {
    x: Option<SideX>,
    y: Option<SideY>,
}

impl Direction {
    // Included for completeness but realistically never going to be used.
    pub const CENTER: Self = Self::new(None, None);
    pub const DOWN: Self = Self::new(None, Some(SideY::Down));
    pub const DOWN_LEFT: Self = Self::new(Some(SideX::Left), Some(SideY::Down));
    pub const DOWN_RIGHT: Self = Self::new(Some(SideX::Right), Some(SideY::Down));
    pub const LEFT: Self = Self::new(Some(SideX::Left), None);
    pub const RIGHT: Self = Self::new(Some(SideX::Right), None);
    pub const UP: Self = Self::new(None, Some(SideY::Up));
    pub const UP_LEFT: Self = Self::new(Some(SideX::Left), Some(SideY::Up));
    pub const UP_RIGHT: Self = Self::new(Some(SideX::Right), Some(SideY::Up));

    pub const fn new(x: Option<SideX>, y: Option<SideY>) -> Self {
        Self { x, y }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        // I wish there was an easier way to do this. Iterating over enums would be *so* nice.
        [
            Self::UP_LEFT,
            Self::UP,
            Self::UP_RIGHT,
            Self::LEFT,
            Self::CENTER,
            Self::RIGHT,
            Self::DOWN_LEFT,
            Self::DOWN,
            Self::DOWN_RIGHT,
        ]
        .into_iter()
    }

    pub fn iter_no_center() -> impl Iterator<Item = Self> {
        Self::iter().filter(|v| v != &Self::CENTER)
    }

    pub const fn offset_x(self) -> isize {
        match self.x {
            Some(v) => v.offset(),
            None => 0,
        }
    }

    pub const fn offset_y(self) -> isize {
        match self.y {
            Some(v) => v.offset(),
            None => 0,
        }
    }
}

fn input(path: impl AsRef<Path>) -> Result<Input> {
    // Just splits the input string into a grid of characters.
    Ok(read_to_string(path)?.lines().map(|v| v.chars().collect()).collect())
}

// Recursive search algorithm to look for characters in a given direction until the stack is empty or the position
// goes out of bounds.
fn search_direction(grid: &[Box<[char]>], x: usize, y: usize, dir: Direction, stack: &[char], index: usize) -> bool {
    stack.get(index).is_none_or(|c| {
        let Some(x) = x.checked_add_signed(dir.offset_x()) else { return false };
        let Some(y) = y.checked_add_signed(dir.offset_y()) else { return false };

        grid.get(y).and_then(|v| v.get(x)).is_some_and(|v| v == c)
            && search_direction(grid, x, y, dir, stack, index + 1)
    })
}

/// Linear search through the entire grid!!! Hooray!!!
fn traverse_grid(grid: &[Box<[char]>], mut f: impl FnMut(&[Box<[char]>], usize, usize)) {
    for y in 0 .. grid.len() {
        for x in 0 .. grid[0].len() {
            f(grid, x, y);
        }
    }
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    const PATTERN: &[char] = &['X', 'M', 'A', 'S'];

    let mut count = 0;

    self::traverse_grid(&self::input(path)?, |grid, x, y| {
        // We need to manually check the first character, since we don't know the direction yet.
        if PATTERN.first().is_none_or(|v| v != &grid[y][x]) {
            return;
        }

        // Check every direction from the target position.
        for dir in Direction::iter_no_center() {
            if search_direction(grid, x, y, dir, PATTERN, 1) {
                count += 1;
            }
        }
    });

    Ok(count as _)
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    const PATTERN: &[char] = &['M', 'A', 'S'];

    let mut centerpoints = BTreeMap::<(usize, usize), usize>::new();

    self::traverse_grid(&self::input(path)?, |grid, x, y| {
        // We need to manually check the first character, since we don't know the direction yet.
        if PATTERN.first().is_none_or(|v| v != &grid[y][x]) {
            return;
        }

        for dir in Direction::iter_no_center().filter(|d| {
            // Only allow diagonal directions.
            d.x.is_some() && d.y.is_some() && search_direction(grid, x, y, *d, PATTERN, 1)
        }) {
            let Some(x) = x.checked_add_signed(dir.offset_x()) else { continue };
            let Some(y) = y.checked_add_signed(dir.offset_y()) else { continue };

            *centerpoints.entry((x, y)).or_default() += 1;
        }
    });

    Ok(centerpoints.into_iter().filter(|(_, v)| *v > 1).count() as _)
}

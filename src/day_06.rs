use std::path::Path;

use anyhow::{Result, bail};

use crate::SolutionResult;
use crate::utility::grid::{Grid2D, Offset2D, Pos2D, read_to_char_grid};

type Input = (Grid2D<char>, Pos2D);

fn input(path: impl AsRef<Path>) -> Result<Input> {
    let mut guard_pos = None;

    let grid = read_to_char_grid(path, |pos, c| match c {
        '#' => Some('#'),
        // We ignore the guard character because we don't care about people, only octothorpes.
        '^' => {
            guard_pos = Some(pos);

            None
        }
        // We also ignore every other character, because again, we only care about octothorpes.
        _ => None,
    })?;

    let Some(guard_pos) = guard_pos else { bail!("missing initial guard position") };

    Ok((grid, guard_pos))
}

const fn turn(direction: u8) -> u8 {
    direction.wrapping_add(1) % 4
}

const fn direction_to_offset(direction: u8) -> Offset2D {
    match direction % 4 {
        0 => Offset2D::new(0, -1),
        1 => Offset2D::new(1, 0),
        2 => Offset2D::new(0, 1),
        3 => Offset2D::new(-1, 0),
        _ => unreachable!(),
    }
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    let (mut grid, mut guard_pos) = self::input(path)?;
    let mut direction = 0;

    // We can just subtract the added characters from the initial cell count to get the touched positions.
    let initial_tiles = grid.cells().filter(|v| v.is_some()).count();

    while grid.size().contains_position(guard_pos) {
        let offset = self::direction_to_offset(direction);
        let Some(next_pos) = guard_pos.offset(offset) else { break };

        match grid.get(next_pos) {
            Some('#') => direction = self::turn(direction),
            // Place an X over all traveled tiles.
            Some('X') | None => {
                grid.set(guard_pos, 'X');
                guard_pos = next_pos;
            }
            Some(c) => bail!("unexpected character {c:?}"),
        }
    }

    let final_tiles = grid.cells().filter(|v| v.is_some()).count();

    Ok((final_tiles - initial_tiles) as _)
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    let (grid, guard_start_pos) = self::input(path)?;
    let mut count: usize = 0;

    // And so begins the slowest implementation so far.
    'outer: for obstacle_pos in grid.iter().filter_map(|(p, v)| (v.is_none() && p != guard_start_pos).then_some(p)) {
        let mut snapshots = Vec::<(u8, Pos2D)>::new();
        let mut direction = 0;
        let mut guard_pos = guard_start_pos;
        let mut grid = grid.clone();

        grid.set(obstacle_pos, 'O');

        'inner: while grid.size().contains_position(guard_pos) {
            let mut new_char = if direction % 2 == 0 { '|' } else { '-' };

            // Assume any passed characters will be ours. Our greed is immeasurable.
            if grid.get(guard_pos).is_some_and(|c| *c != new_char) {
                new_char = '+';
            }

            grid.set(guard_pos, new_char);

            // Check for bounds.
            let offset = self::direction_to_offset(direction);
            let Some(next_pos) = guard_pos.offset(offset) else { continue 'outer };
            if !grid.size().contains_position(next_pos) {
                continue 'outer;
            }

            if let Some('#' | 'O') = grid.get(next_pos) {
                direction = self::turn(direction);

                continue 'inner;
            }

            guard_pos = next_pos;

            // If we've already passed a given point with the current direction, we've entered a loop.
            if snapshots.iter().any(|(d, p)| *p == guard_pos && *d == direction) {
                break 'inner;
            } else {
                snapshots.push((direction, guard_pos));
            }
        }

        count += 1;
    }

    Ok(count as _)
}

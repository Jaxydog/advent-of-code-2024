use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::Result;

use crate::SolutionResult;
use crate::utility::grid::{Grid2D, Offset2D, Pos2D, read_to_char_grid};

type Input = Grid2D<char>;

fn input(path: impl AsRef<Path>) -> Result<Input> {
    read_to_char_grid(path, |_, c| c.is_ascii_alphanumeric().then_some(c))
}

/// Returns a list of valid anode positions on either side of a pair of signals.
fn anodes_iter(
    grid: &Grid2D<char>,
    signal: Option<&char>,
    position_1: Pos2D,
    position_2: Pos2D,
    max_steps: Option<usize>,
) -> impl Iterator<Item = Pos2D> {
    // Calculate the difference of the two positions to use as an offset for anode positions.
    let delta_x = position_2.x() as isize - position_1.x() as isize;
    let delta_y = position_2.y() as isize - position_1.y() as isize;

    // This isn't exact, but it's good enough. Thank you Pythagoras.
    let max_steps = max_steps.unwrap_or_else(|| {
        let w = grid.size().w().get();
        let h = grid.size().h().get();

        // Always round up so that we don't truncate and possibly miss a step.
        (((w * w) + (h * h)) as f64).sqrt().ceil() as usize
    });

    // Iterator of increasingly larger offsets that are used to map anodes.
    let offsets = (0 ..= max_steps as isize).map(move |v| Offset2D::new(delta_x * v, delta_y * v));
    let anodes_1 = offsets.clone().filter_map(move |v| position_1.offset(-v));
    let anodes_2 = offsets.filter_map(move |v| position_2.offset(v));

    anodes_1.chain(anodes_2).filter(move |v| {
        // We chain and filter out any positions that do not fit within the grid, or that overlap a matching signal.
        v.is_contained_within(grid.size()) && grid.get(*v).is_none_or(|v| signal.is_none_or(|u| v != u))
    })
}

/// Count all unique anodes present in the given grid of signals.
fn count_anodes(grid: &Grid2D<char>, max_steps: Option<usize>, no_overlap: bool) -> usize {
    // Group all signal positions by their characters.
    let mut signals = HashMap::<char, HashSet<Pos2D>>::new();

    for (position, signal) in grid.iter().filter_map(|(p, c)| c.map(move |c| (p, c))) {
        signals.entry(signal).or_default().insert(position);
    }

    // And then track all unique anode positions.
    let mut anodes = HashSet::<Pos2D>::new();

    for (signal, position_1, position_2) in signals.iter().flat_map(|(signal, set)| {
        // Only allow positions that are non-equal to pass through.
        set.iter().flat_map(move |a| set.iter().filter_map(move |b| (a != b).then_some((signal, *a, *b))))
    }) {
        anodes.extend(self::anodes_iter(grid, no_overlap.then_some(signal), position_1, position_2, max_steps));
    }

    anodes.len()
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    self::input(path).map(|grid| self::count_anodes(&grid, Some(1), true) as _)
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    self::input(path).map(|grid| self::count_anodes(&grid, None, false) as _)
}

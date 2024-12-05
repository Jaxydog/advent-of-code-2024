use std::io::{Write, stdout};

use anyhow::{Result, anyhow};

mod day_1;
mod day_2;
mod day_3;
mod day_4;

// I use a type alias here in case I ever need to change the integer size.
pub type SolutionResult = Result<u64>;

macro_rules! match_arguments {
    (($argument_day:expr, $argument_solution:expr, $argument_example:expr) {$(
        $specified_day:literal => $specified_module:ident
    ),+ $(,)? }) => {{
        // Informs the function what file path should be used to access its data.
        #[inline]
        fn __solution_path(number: u8, example: bool) -> impl ::std::convert::AsRef<::std::path::Path> {
            ::std::format!("./data/day_{number}{}.txt", if example { "_example" } else { "" })
        }

        match ($argument_day, $argument_solution) {
            // We auto-fill both solution matches for each given day, under the assumption that I actually finished both
            // solutions for every given day. Let's hope I have both the motivation and skill, shall we?
            $(
                ($specified_day, 1) => $crate::$specified_module::solution_1(__solution_path($specified_day, $argument_example)),
                ($specified_day, 2) => $crate::$specified_module::solution_2(__solution_path($specified_day, $argument_example)),
            )+
            // I was today years old when I figured out that you can use this syntax (referring to the `0 | 3..`).
            // This is probably less efficient than just checking if it's 1 or 2 directly but *oh well*.
            (_, 0 | 3 ..) => ::anyhow::bail!("the solution must be either 1 or 2"),
            _ => ::anyhow::bail!("the given day number has not been mapped to any solutions"),
        }
    }};
}

fn main() -> Result<()> {
    // This is for the sake of debugging errors or panics, without having to remember to type out `RUST_BACKTRACE=1`
    // before every command, while I'm trying to finish each day in a timely manner.
    //
    // Safety: Nothing else is currently accessing the environment, as this is the first line.
    unsafe { std::env::set_var("RUST_BACKTRACE", "1") };

    let mut arguments = std::env::args().skip(1);

    // I'm gonna give `clap` a run for its money with this one /j
    let expected_day: u8 = arguments.next().ok_or_else(|| anyhow!("missing day"))?.parse()?;
    let expected_solution: u8 = arguments.next().ok_or_else(|| anyhow!("missing solution"))?.parse()?;
    let use_examples = arguments.next().is_some_and(|v| v.parse::<u8>().is_ok_and(|v| v == 1));

    let solution = match_arguments!((expected_day, expected_solution, use_examples) {
        1 => day_1,
        2 => day_2,
        3 => day_3,
        4 => day_4,
    })?;

    stdout().write_fmt(format_args!("{solution}\n")).map_err(Into::into)
}

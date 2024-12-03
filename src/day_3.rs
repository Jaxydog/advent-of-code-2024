use std::error::Error;
use std::fs::read_to_string;
use std::iter::Peekable;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Result, bail};

use crate::SolutionResult;

/// Oh yeah, you *love* to see a near direct pass-through of the entire file.
type Input = Box<str>;

/// Represents a multiplication operation (e.g., `mul(24, 48)`)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Multiply(u16, u16);

impl Multiply {
    /// For convenience.
    pub const fn get(self) -> u64 {
        self.0 as u64 * self.1 as u64
    }
}

impl FromStr for Multiply {
    type Err = anyhow::Error;

    // Behold! Stupid string parsing.
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut iterator = string.chars().peekable();

        advance_str(&mut iterator, "mul(")?;

        let lhs = advance_number(&mut iterator)?;

        advance_str(&mut iterator, ",")?;

        let rhs = advance_number(&mut iterator)?;

        advance_str(&mut iterator, ")")?;

        Ok(Self(lhs, rhs))
    }
}

/// Represents either `do()` or `don't()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Enable {
    Do,
    Dont,
}

impl FromStr for Enable {
    type Err = anyhow::Error;

    // Behold! Stupid string parsing, but again.
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if advance_str(&mut string.chars(), "don't()").is_ok() {
            Ok(Self::Dont)
        } else if advance_str(&mut string.chars(), "do()").is_ok() {
            Ok(Self::Do)
        } else {
            bail!("the given string does not start with a valid operation");
        }
    }
}

fn input(path: impl AsRef<Path>) -> Result<Input> {
    Ok(read_to_string(path)?.into_boxed_str())
}

/// Advances the given iterator by the length of the given string, ensuring that the returned characters match the
/// string exactly.
fn advance_str(iterator: &mut impl Iterator<Item = char>, string: &str) -> Result<()> {
    for index in 0 .. string.len() {
        match iterator.next() {
            Some(c) if c == string.chars().nth(index).unwrap() => continue,
            Some(c) => bail!("unexpected character '{c}'"),
            None => bail!("unexpected end of iterator"),
        };
    }

    Ok(())
}

/// Advances the given iterator until a non-digit character is encountered, then returns the parsed number.
fn advance_number<T>(iterator: &mut Peekable<impl Iterator<Item = char>>) -> Result<T>
where
    // Awful bound requirements, but whatever.
    T: FromStr<Err: Error + Send + Sync + 'static>,
{
    // Realistically this will never be more than 3 due to the nature of our input.
    let mut digits = String::with_capacity(3);

    // God bless `.peek`.
    while iterator.peek().is_some_and(|v| v.is_ascii_digit()) {
        digits.push(iterator.next().unwrap());
    }

    Ok(digits.parse()?)
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    let string = self::input(path)?;

    let mut total = 0;

    for (index, character) in string.char_indices() {
        if character != 'm' {
            continue;
        }

        if let Ok(multiply) = string[index ..].parse::<Multiply>() {
            total += multiply.get();
        }
    }

    Ok(total)
}

pub fn solution_2(path: impl AsRef<Path>) -> SolutionResult {
    let string = self::input(path)?;

    let mut last_enable = None;
    let mut total = 0;

    // Holy indentation Batman!
    for (index, character) in string.char_indices() {
        match character {
            'd' => {
                if let Ok(enable) = string[index ..].parse::<Enable>() {
                    last_enable = Some(enable);
                }
            }
            'm' => {
                if let Ok(multiply) = string[index ..].parse::<Multiply>() {
                    if last_enable.is_none_or(|v| v == Enable::Do) {
                        total += multiply.get();
                    }
                }
            }
            _ => continue,
        }
    }

    Ok(total)
}

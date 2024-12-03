use std::error::Error;
use std::fs::read_to_string;
use std::iter::Peekable;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Result, bail};

use crate::SolutionResult;

// Hello, chat. Today we're going to misuse the `FromStr` trait because I like using `a.parse()`.
//
// We will NOT be checking the entire provided string, we WILL be only checking the start of it.
// And that's okay! Break rules, live stupidly, and eat rocks.

/// Oh yeah, you *love* to see a near direct pass-through of the entire file.
type Input = Box<str>;

/// Represents a multiplication operation (e.g., `mul(24,48)`)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Multiply(u16, u16);

impl Multiply {
    /// For convenience. *My* convenience, not yours.
    pub const fn get(self) -> u64 {
        // Once again casting *before* the operation to make sure that we have enough space to really stretch out and
        // expand after multiplying.
        self.0 as u64 * self.1 as u64
    }
}

impl FromStr for Multiply {
    type Err = anyhow::Error;

    // Behold! Stupid string parsing. See top-level comment for why this is especially bad.
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

    // Behold! Stupid string parsing, but again. See the last `FromStr` implementation comment for why this is, also,
    // especially bad.
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
    // This is like, fine, I guess? It works and isn't downright insulting to read, which is all I really care about
    // for these challenges.
    for index in 0 .. string.len() {
        match iterator.next() {
            // This doesn't *have* to use `continue`, it could just be an empty block.
            // But I like the control, the sense of power. I command the code and it listens and bends to my will.
            //
            // Okay, maybe it's not really that deep.
            // I don't think it actually matters which I pick, and I like having the comma for consistency with the
            // other match arms.
            //
            // Also, the character at this index is guaranteed to exist, so unwrapping is okay.
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

    // God bless `.peek()` o7
    while iterator.peek().is_some_and(|v| v.is_ascii_digit()) {
        // We just verified that `.next()` will return a value, so unwrapping is fine.
        digits.push(iterator.next().unwrap());
    }

    Ok(digits.parse()?)
}

pub fn solution_1(path: impl AsRef<Path>) -> SolutionResult {
    let string = self::input(path)?;

    let mut total = 0;

    for (index, character) in string.char_indices() {
        // We don't care about any characters other than 'm'. Not personally, just in this context.
        // Well.. Maybe personally.
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

    for (index, character) in string.char_indices() {
        // This time we also care about 'd'.
        // 'm' was just getting WAY too much attention before, and we need to really spread out the love, y'know?
        match character {
            'd' => {
                if let Ok(enable) = string[index ..].parse::<Enable>() {
                    last_enable = Some(enable);
                }
            }
            // Holy indentation Batman!
            'm' => {
                if let Ok(multiply) = string[index ..].parse::<Multiply>() {
                    // If it hasn't been set initially we can just assume that the operations are fine to use.
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

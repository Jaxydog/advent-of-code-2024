use anyhow::{Result, bail};

mod day_1;
mod day_2;

fn main() -> Result<()> {
    let Some(day) = std::env::args().nth(1) else {
        bail!("missing day number");
    };
    let Some(part) = std::env::args().nth(2) else {
        bail!("missing part number");
    };

    macro_rules! match_days {
        ($($day:literal => $module:ident,)+) => {
            match (day.parse::<u8>()?, part.parse::<u8>()?) {
                $(
                    ($day, 1) => self::$module::part_1(),
                    ($day, 2) => self::$module::part_2(),
                )+
                (_, n) if n != 1 && n != 2 => bail!("expected 1 or 2 for part number"),
                (..) => bail!("unknown or unfinished day number"),
            }
        };
    }

    match_days! {
        1 => day_1,
        2 => day_2,
    }
}

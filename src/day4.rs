use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use strum::EnumString;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Year(u16);

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct BirthYear(Year);

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct IssueYear(Year);

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct ExpirationYear(Year);

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Height {
    Cm(u8),
    In(u8),
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct HairColor([char; 6]);

#[derive(Clone, Copy, EnumString)]
enum EyeColor {
    #[strum(serialize = "amb")]
    Amb,
    #[strum(serialize = "blu")]
    Blu,
    #[strum(serialize = "brn")]
    Brn,
    #[strum(serialize = "gry")]
    Gry,
    #[strum(serialize = "grn")]
    Grn,
    #[strum(serialize = "hzl")]
    Hzl,
    #[strum(serialize = "oth")]
    Oth,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Pid([char; 9]);

#[allow(dead_code)]
struct ValidPassport {
    byr: BirthYear,
    iyr: IssueYear,
    eyr: ExpirationYear,
    hgt: Height,
    hcl: HairColor,
    ecl: EyeColor,
    pid: Pid,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(passport: &str) -> Result<Self> {
        fn parse_val(fields: &[(&str, &str)], key: &str) -> Option<String> {
            fields
                .iter()
                .find_map(|&(k, v)| (k == key).then(|| v.to_string()))
        }

        let fields = passport
            .split_ascii_whitespace()
            .map(|kv| {
                kv.split_once(':')
                    .ok_or_else(|| anyhow!("invalid key-value '{kv}'"))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            byr: parse_val(&fields, "byr")
                .ok_or_else(|| anyhow!("missing 'byr' in passport '{passport}'"))?,
            iyr: parse_val(&fields, "iyr")
                .ok_or_else(|| anyhow!("missing 'iyr' in passport '{passport}'"))?,
            eyr: parse_val(&fields, "eyr")
                .ok_or_else(|| anyhow!("missing 'eyr' in passport '{passport}'"))?,
            hgt: parse_val(&fields, "hgt")
                .ok_or_else(|| anyhow!("missing 'hgt' in passport '{passport}'"))?,
            hcl: parse_val(&fields, "hcl")
                .ok_or_else(|| anyhow!("missing 'hcl' in passport '{passport}'"))?,
            ecl: parse_val(&fields, "ecl")
                .ok_or_else(|| anyhow!("missing 'ecl' in passport '{passport}'"))?,
            pid: parse_val(&fields, "pid")
                .ok_or_else(|| anyhow!("missing 'pid' in passport '{passport}'"))?,
            cid: parse_val(&fields, "cid"),
        })
    }
}

impl FromStr for Year {
    type Err = Error;

    fn from_str(year: &str) -> Result<Self> {
        if year.len() != 4 {
            bail!("year '{year}' is not four digits");
        }

        Ok(Self(year.parse()?))
    }
}

impl TryFrom<Year> for BirthYear {
    type Error = Error;

    fn try_from(year: Year) -> Result<Self> {
        const VALID_BYR: RangeInclusive<u16> = 1920..=2002;

        if !VALID_BYR.contains(&year.0) {
            bail!(
                "birth year '{}' is not in valid range '{VALID_BYR:?}'",
                year.0,
            );
        }

        Ok(Self(year))
    }
}

impl TryFrom<Year> for IssueYear {
    type Error = Error;

    fn try_from(year: Year) -> Result<Self> {
        const VALID_IYR: RangeInclusive<u16> = 2010..=2020;

        if !VALID_IYR.contains(&year.0) {
            bail!(
                "issue year '{}' is not in valid range '{VALID_IYR:?}'",
                year.0,
            );
        }

        Ok(Self(year))
    }
}

impl TryFrom<Year> for ExpirationYear {
    type Error = Error;

    fn try_from(year: Year) -> Result<Self> {
        const VALID_EYR: RangeInclusive<u16> = 2020..=2030;

        if !VALID_EYR.contains(&year.0) {
            bail!(
                "expiration year '{}' is not in valid range '{VALID_EYR:?}'",
                year.0,
            );
        }

        Ok(Self(year))
    }
}

impl FromStr for Height {
    type Err = Error;

    fn from_str(height: &str) -> Result<Self> {
        if let Some(height) = height.strip_suffix("cm") {
            const VALID_CM: RangeInclusive<u8> = 150..=193;

            let height = height.parse()?;
            if !VALID_CM.contains(&height) {
                bail!("centimeter height '{height}' is not in valid range '{VALID_CM:?}'");
            }

            Ok(Self::Cm(height))
        } else if let Some(height) = height.strip_suffix("in") {
            const VALID_IN: RangeInclusive<u8> = 59..=76;

            let height = height.parse()?;
            if !VALID_IN.contains(&height) {
                bail!("inches height '{height}' is not in valid range '{VALID_IN:?}'");
            }

            Ok(Self::In(height))
        } else {
            bail!("invalid height '{height}'");
        }
    }
}

impl FromStr for HairColor {
    type Err = Error;

    fn from_str(color: &str) -> Result<Self> {
        let color = color
            .strip_prefix('#')
            .ok_or_else(|| anyhow!("invalid hair color '{color}'"))?;

        if color.chars().any(|c: char| !c.is_ascii_hexdigit()) {
            bail!("invalid character in hair color '{color}'");
        }

        let color = color
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|color: Vec<_>| color.into_iter().collect::<String>())
            .map_err(|color| anyhow!("color '{color}' is not six characters"))?;

        Ok(Self(color))
    }
}

impl FromStr for Pid {
    type Err = Error;

    fn from_str(pid: &str) -> Result<Self> {
        if pid.chars().any(|c| !c.is_ascii_digit()) {
            bail!("invalid character in passport ID '{pid}'");
        }

        let pid = pid
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|pid: Vec<_>| pid.into_iter().collect::<String>())
            .map_err(|pid| anyhow!("pid '{pid}' is not nine characters"))?;

        Ok(Self(pid))
    }
}

impl TryFrom<Passport> for ValidPassport {
    type Error = Error;

    fn try_from(passport: Passport) -> Result<Self> {
        let valid = Self {
            byr: passport.byr.parse::<Year>()?.try_into()?,
            iyr: passport.iyr.parse::<Year>()?.try_into()?,
            eyr: passport.eyr.parse::<Year>()?.try_into()?,
            hgt: passport.hgt.parse()?,
            hcl: passport.hcl.parse()?,
            ecl: passport.ecl.parse()?,
            pid: passport.pid.parse()?,
            cid: passport.cid,
        };

        Ok(valid)
    }
}

fn main() -> Result<()> {
    let passports = fs::read_to_string("in/day4.txt")?
        .split("\n\n")
        .flat_map(Passport::from_str)
        .collect::<Vec<_>>();

    {
        let start = Instant::now();
        let part1 = passports.len();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 250);
    };

    {
        let start = Instant::now();
        let part2 = passports
            .into_iter()
            .flat_map(ValidPassport::try_from)
            .count();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 158);
    };

    Ok(())
}

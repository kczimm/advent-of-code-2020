use input;
use std::io::Result;
fn main() -> Result<()> {
    let input = input::load_file("src/day4/input.txt")?;

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));

    Ok(())
}

pub fn part1(input: &str) -> usize {
    Passport::from_batch(input)
        .into_iter()
        .filter(|p| p.fields_present())
        .count()
}

pub fn part2(input: &str) -> usize {
    Passport::from_batch(input)
        .into_iter()
        .filter(|p| p.fields_present() & p.is_valid())
        .count()
}

#[derive(Debug)]
pub enum Field<'a> {
    BirthYear(&'a str),
    IssueYear(&'a str),
    ExpirationYear(&'a str),
    Height(&'a str),
    HairColor(&'a str),
    EyeColor(&'a str),
    PassportID(&'a str),
    CountryID(&'a str),
}

impl<'a> Field<'a> {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::BirthYear(year) => match year.chars().count() {
                4 => match year.parse::<u32>() {
                    Ok(year) => (1920 <= year) & (year <= 2002),
                    Err(_) => false,
                },
                _ => false,
            },
            Self::IssueYear(year) => match year.chars().count() {
                4 => match year.parse::<u32>() {
                    Ok(year) => (2010 <= year) & (year <= 2020),
                    Err(_) => false,
                },
                _ => false,
            },
            Self::ExpirationYear(year) => match year.chars().count() {
                4 => match year.parse::<u32>() {
                    Ok(year) => (2020 <= year) & (year <= 2030),
                    Err(_) => false,
                },
                _ => false,
            },
            Self::Height(height) => {
                let num_chars = height.chars().count();
                let unit = &height[num_chars - 2..];
                match (&height[0..num_chars - 2]).parse::<u8>() {
                    Ok(height) => match unit {
                        "in" => (59 <= height) & (height <= 76),
                        "cm" => (150 <= height) & (height <= 193),
                        _ => false,
                    },
                    Err(_) => false,
                }
            }
            // a # followed by exactly six characters 0-9 or a-f.
            Self::HairColor(color) => {
                if color.chars().count() != 7 {
                    return false;
                }

                let mut chars = color.chars();
                match chars.next() {
                    Some(c) => {
                        if c != '#' {
                            return false;
                        }
                    }
                    None => return false,
                }

                chars.all(|c| match c {
                    '0'..='9' => true,
                    'a'..='f' => true,
                    _ => false,
                })
            }
            Self::EyeColor(color) => match *color {
                // exactly one of: amb blu brn gry grn hzl oth
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            Self::PassportID(id) => {
                // a nine-digit number, including leading zeroes.
                match id.chars().count() {
                    9 => id.parse::<u32>().is_ok(),
                    _ => false,
                }
            }
            Self::CountryID(_) => true,
        }
    }

    pub fn from(line: &'a str) -> Option<Self> {
        // separate key/value by ':'
        match line.find(':') {
            Some(i) => {
                let key = &line[0..i];
                let value = &line[i + 1..];
                match key {
                    "byr" => Some(Self::BirthYear(value)),
                    "iyr" => Some(Self::IssueYear(value)),
                    "eyr" => Some(Self::ExpirationYear(value)),
                    "hgt" => Some(Self::Height(value)),
                    "hcl" => Some(Self::HairColor(value)),
                    "ecl" => Some(Self::EyeColor(value)),
                    "pid" => Some(Self::PassportID(value)),
                    "cid" => Some(Self::CountryID(value)),
                    k => panic!("bad key: {}", k),
                }
            }
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct Passport<'a>(Vec<Field<'a>>);

impl<'a> Passport<'a> {
    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|f| f.is_valid())
    }

    pub fn fields_present(&self) -> bool {
        let mut mask = 0u8;
        for field in self.0.iter() {
            mask = match field {
                Field::BirthYear(_) => mask | 1 << 0,
                Field::IssueYear(_) => mask | 1 << 1,
                Field::ExpirationYear(_) => mask | 1 << 2,
                Field::Height(_) => mask | 1 << 3,
                Field::HairColor(_) => mask | 1 << 4,
                Field::EyeColor(_) => mask | 1 << 5,
                Field::PassportID(_) => mask | 1 << 6,
                Field::CountryID(_) => mask | 1 << 7,
            }
        }

        // only CountryID can be missing
        mask | 1 << 7 == 255
    }

    pub fn from_text(text: &'a str) -> Self {
        let mut p = vec![];

        let spaces_newlines_at = text.char_indices().filter_map(|(i, c)| match c {
            ' ' | '\n' => Some(i),
            _ => None,
        });

        let mut prev = 0;

        for i in spaces_newlines_at {
            let field = &text[prev..i];

            match Field::from(field) {
                Some(f) => p.push(f),
                None => {}
            }

            prev = i + 1;
        }

        let field = &text[prev..];

        match Field::from(field) {
            Some(f) => p.push(f),
            None => {}
        }

        Self(p)
    }

    pub fn from_batch(batch: &'a str) -> Vec<Passport> {
        batch
            .split("\n\n")
            .map(|part| Self::from_text(part))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_hair_color_isvalid() {
        assert!(Field::HairColor("#123abc").is_valid());
        assert!(!Field::HairColor("#123abF").is_valid());
        assert!(!Field::HairColor("#1234abc").is_valid());
        assert!(!Field::HairColor("123abc").is_valid());
        assert!(!Field::HairColor("#123abg").is_valid());
    }

    #[test]
    fn test_fields() {
        assert!(Field::BirthYear("2002").is_valid());
        assert!(!Field::BirthYear("2003").is_valid());

        assert!(Field::Height("60in").is_valid());
        assert!(Field::Height("190cm").is_valid());
        assert!(!Field::Height("190in").is_valid());
        assert!(!Field::Height("190").is_valid());

        assert!(Field::HairColor("#123abc").is_valid());
        assert!(!Field::HairColor("#123abz").is_valid());
        assert!(!Field::HairColor("123abc").is_valid());

        assert!(Field::EyeColor("brn").is_valid());
        assert!(!Field::EyeColor("wat").is_valid());

        assert!(Field::PassportID("000000001").is_valid());
        assert!(!Field::PassportID("0123456789").is_valid());
    }

    #[test]
    fn test_part1() {
        let batch = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        let num_valid = part1(batch);

        assert_eq!(num_valid, 2);
    }

    #[test]
    fn test_invalid_passports() {
        let batch = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";

        let passports = Passport::from_batch(batch);

        for passport in passports {
            assert!(!passport.is_valid());
        }
    }

    #[test]
    fn test_valid_passports() {
        let batch = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports = Passport::from_batch(batch);

        for passport in passports {
            // assert!(passport.is_valid());
            passport.is_valid();
        }
    }
}

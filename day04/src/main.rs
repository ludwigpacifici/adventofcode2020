use std::io::prelude::*;
use std::{collections::HashMap, io};
use std::{fs::File, str::FromStr};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("part one: {:?}", part_one(&input));
    println!("part two: {:?}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|p| PassportDto::from_str(p).is_ok())
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|p| Passport::from_str(p).is_ok())
        .count()
}

struct PassportDto {
    birth_year: String,         // byr
    issue_year: String,         // iyr
    expiration_year: String,    // eyr
    height: String,             // hgt
    hair_color: String,         // hcl
    eye_color: String,          // ecl
    passport_id: String,        // pid
    country_id: Option<String>, // cid
}

impl FromStr for PassportDto {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s
            .lines()
            .flat_map(|l| l.split_whitespace())
            .fold(HashMap::new(), |mut acc, kv| {
                let mut it = kv.split(':');
                let k = it.next().expect("Cannot read key");
                let v = it.next().expect("Cannot read value");
                acc.insert(k, v);
                acc
            });

        let birth_year = p
            .get("byr")
            .map(|s| s.to_string())
            .ok_or("Birth year (byr) not found.".to_string())?;

        let issue_year = p
            .get("iyr")
            .map(|s| s.to_string())
            .ok_or("Issue year (iyr) not found.".to_string())?;

        let expiration_year = p
            .get("eyr")
            .map(|s| s.to_string())
            .ok_or("Expiration year (eyr) not found.".to_string())?;

        let height = p
            .get("hgt")
            .map(|s| s.to_string())
            .ok_or("Height (hgt) not found.".to_string())?;

        let hair_color = p
            .get("hcl")
            .map(|s| s.to_string())
            .ok_or("Hair color (hcl) not found.".to_string())?;

        let eye_color = p
            .get("ecl")
            .map(|s| s.to_string())
            .ok_or("Eye color (ecl) not found.".to_string())?;

        let passport_id = p
            .get("pid")
            .map(|s| s.to_string())
            .ok_or("Passport id (pid) not found.".to_string())?;

        let country_id = p.get("cid").map(|s| s.to_string());

        Ok(PassportDto {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        })
    }
}

struct Passport {
    birth_year: u64,
    issue_year: u64,
    expiration_year: u64,
    height: (u64, String),
    hair_color: String,
    eye_color: String,
    passport_id: u64,
    country_id: Option<String>,
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = PassportDto::from_str(s)?;

        let birth_year = p.birth_year.parse::<u64>().map_err(|e| e.to_string())?;

        if !(1920 <= birth_year && birth_year <= 2002) {
            return Err("Invalid birth year".to_string());
        }

        let issue_year = p.issue_year.parse::<u64>().map_err(|e| e.to_string())?;

        if !(2010 <= issue_year && issue_year <= 2020) {
            return Err("Invalid issue year".to_string());
        }

        let expiration_year = p
            .expiration_year
            .parse::<u64>()
            .map_err(|e| e.to_string())?;

        if !(2020 <= expiration_year && expiration_year <= 2030) {
            return Err("Invalid expiration year".to_string());
        }

        let (height_value, height_unit) = p.height.split_at(p.height.len() - 2);
        let height_value = height_value.parse::<u64>().map_err(|e| e.to_string())?;

        if height_unit == "cm" && !(150 <= height_value && height_value <= 193) {
            return Err("Invalid height in 'cm'".to_string());
        } else if height_unit == "in" && !(59 <= height_value && height_value <= 76) {
            return Err("Invalid height in 'in'".to_string());
        } else if height_unit != "cm" && height_unit != "in" {
            return Err("Invalid height unit".to_string());
        }

        if p.hair_color.len() != 7 {
            return Err("Hair color should be of length 7".to_string());
        }
        let mut it = p.hair_color.chars();
        let hash = it.next().expect("Cannot read first char of hair color");
        if hash != '#' {
            return Err("Hair color should stast with '#'".to_string());
        }
        if it.all(|d| d.is_digit(16)) == false {
            return Err("Hair color should be an hexadecimal".to_string());
        }

        if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|&c| c == p.eye_color)
            == false
        {
            return Err("Eye color is invalid".to_string());
        }

        if p.passport_id.len() != 9 {
            return Err("Passport id should be of length 9".to_string());
        }

        let passport_id = p.passport_id.parse::<u64>().map_err(|e| e.to_string())?;

        Ok(Passport {
            birth_year,
            issue_year,
            expiration_year,
            height: (height_value, height_unit.to_string()),
            hair_color: p.hair_color,
            eye_color: p.eye_color,
            passport_id,
            country_id: p.country_id,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn part_two_test() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

        assert_eq!(part_two(input), 0);

        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(part_two(input), 4);
    }
}

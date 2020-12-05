use aoc2020::parse_newline_sep;

use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;
use thiserror::Error;
use regex::Regex;
use lazy_static::lazy_static;


struct Passport{
    ecl: String,
    pid: String,
    eyr: String,
    hcl: String,
    byr: String,
    iyr: String,
    hgt: String,
}

lazy_static!{
    static ref HGT_CM: Regex = Regex::new(r"^(\d\d\d)cm$").unwrap();
    static ref HGT_IN: Regex = Regex::new(r"^(\d\d)in$").unwrap();
    static ref HCL: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

impl Passport {
    fn complete(&self) -> bool {
        [&self.ecl, &self.pid, &self.eyr, &self.hcl, &self.byr, &self.iyr, &self.hgt]
            .iter()
            .all(|field| *field != "")
    }

    fn valid(&self) -> bool {
        self.valid_byr() && self.valid_iyr() && self.valid_eyr() && self.valid_hgt() &&
            self.valid_hcl() && self.valid_ecl() && self.valid_pid()
    }

    fn valid_byr(&self) -> bool {
        match self.byr.parse::<usize>(){
            Ok(n) => 1920 <= n && n <= 2002,
            Err(_) => false
        }
    }

    fn valid_iyr(&self) -> bool {
        match self.iyr.parse::<usize>(){
            Ok(n) => 2010 <= n && n <= 2020,
            Err(_) => false
        }
    }
    
    fn valid_eyr(&self) -> bool {
        match self.eyr.parse::<usize>(){
            Ok(n) => 2020 <= n && n <= 2030,
            Err(_) => false
        }
    }

    fn valid_hgt(&self) -> bool {
        match HGT_CM.captures(&self.hgt){
            Some(captures) => {
                let height = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                150 <= height && height <= 193
            }
            None => {
                match HGT_IN.captures(&self.hgt){
                    Some(captures) => {
                        let height = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        59 <= height && height <= 76
                    }
                    None => {
                        false
                    }
                }
            }
        }
    }

    fn valid_hcl(&self) -> bool {
        HCL.is_match(&self.hcl)
    }

    fn valid_ecl(&self) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|c| c.to_string() == self.ecl)
    }

    fn valid_pid(&self) -> bool {
        PID.is_match(&self.pid)
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(input: &str) -> Result<Passport, Self::Err> {
        let mut passport_fields = HashMap::new();

        for field in input.split_whitespace() {
            let chunks: Vec<&str> = field.split(':').collect();
            passport_fields.insert(chunks[0], chunks[1]);
        }

        passport_fields.insert("cid", "North Pole (probably, I guess)");

        Ok(Passport{
            ecl: passport_fields.get("ecl").unwrap_or(&"").to_string(),
            pid: passport_fields.get("pid").unwrap_or(&"").to_string(),
            eyr: passport_fields.get("eyr").unwrap_or(&"").to_string(),
            hcl: passport_fields.get("hcl").unwrap_or(&"").to_string(),
            byr: passport_fields.get("byr").unwrap_or(&"").to_string(),
            iyr: passport_fields.get("iyr").unwrap_or(&"").to_string(),
            hgt: passport_fields.get("hgt").unwrap_or(&"").to_string(),
        })
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let complete_count = parse_newline_sep::<Passport>(input)
                        ?.filter(|p| p.complete())
                        .count(); 
    println!("The answer to part one is {}", complete_count);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let valid_count = parse_newline_sep::<Passport>(input)
                        ?.filter(|p| p.complete() && p.valid())
                        .count(); 
    println!("The answer to part twp is {}", valid_count);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]

#[test]
fn test_complete_passport(){
    let good_passport_str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd \
        byr:1937 iyr:2017 cid:147 hgt:183cm";
    let good_passport = Passport::from_str(good_passport_str).unwrap();
    assert!(good_passport.complete());

    let bad_passport_str = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 \
        hcl:#cfa07d byr:1929";
    let bad_passport = Passport::from_str(bad_passport_str).unwrap();
    assert!(!bad_passport.complete());
}

#[test]
fn test_valid_byr(){
    let good_passport = Passport::from_str("byr:1980").unwrap();
    assert!(good_passport.valid_byr());

    let bad_passport = Passport::from_str("byr:2030").unwrap();
    assert!(!bad_passport.valid_byr());

    let bad_passport = Passport::from_str("byr:twentytwenty").unwrap();
    assert!(!bad_passport.valid_byr());

    let bad_passport = Passport::from_str("").unwrap();
    assert!(!bad_passport.valid_byr());
}

#[test]
fn test_valid_iyr(){
    let good_passport = Passport::from_str("iyr:2017").unwrap();
    assert!(good_passport.valid_iyr());

    let bad_passport = Passport::from_str("iyr:2003").unwrap();
    assert!(!bad_passport.valid_iyr());
}

#[test]
fn test_valid_eyr(){
    let good_passport = Passport::from_str("eyr:2025").unwrap();
    assert!(good_passport.valid_eyr());

    let bad_passport = Passport::from_str("eyr:2040").unwrap();
    assert!(!bad_passport.valid_eyr());
}

#[test]
fn test_valid_hgt(){
    // cm
    let good_passport = Passport::from_str("hgt:180cm").unwrap();
    assert!(good_passport.valid_hgt());

    let bad_passport = Passport::from_str("hgt:120cm").unwrap();
    assert!(!bad_passport.valid_hgt());

    let bad_passport = Passport::from_str("hgt:20cm").unwrap();
    assert!(!bad_passport.valid_hgt());

    // in
    let good_passport = Passport::from_str("hgt:70in").unwrap();
    assert!(good_passport.valid_hgt());

    let bad_passport = Passport::from_str("hgt:90in").unwrap();
    assert!(!bad_passport.valid_hgt());

    let bad_passport = Passport::from_str("hgt:6534in").unwrap();
    assert!(!bad_passport.valid_hgt());

    // other
    let bad_passport = Passport::from_str("hgt:170groats").unwrap();
    assert!(!bad_passport.valid_hgt());
}

#[test]
fn test_valid_hcl(){
    let good_passport = Passport::from_str("hcl:#3a2bff").unwrap();
    assert!(good_passport.valid_hcl());

    let bad_passport = Passport::from_str("hcl:#colour").unwrap();
    assert!(!bad_passport.valid_hcl());

    let bad_passport = Passport::from_str("hcl:red").unwrap();
    assert!(!bad_passport.valid_hcl());
}

#[test]
fn test_valid_ecl(){
    let good_passport = Passport::from_str("ecl:amb").unwrap();
    assert!(good_passport.valid_ecl());

    let bad_passport = Passport::from_str("ecl:amber").unwrap();
    assert!(!bad_passport.valid_ecl());

    let bad_passport = Passport::from_str("ecl:1234").unwrap();
    assert!(!bad_passport.valid_ecl());
}

#[test]
fn test_valid_pid(){
    let good_passport = Passport::from_str("pid:123456789").unwrap();
    assert!(good_passport.valid_pid());

    let bad_passport = Passport::from_str("pid:1234567890").unwrap();
    assert!(!bad_passport.valid_pid());

    let bad_passport = Passport::from_str("pid:my_passport").unwrap();
    assert!(!bad_passport.valid_pid());
}

#[test]
fn test_valid_passport(){
    let good_passport_str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 \
        hcl:#623a2f";
    let good_passport = Passport::from_str(good_passport_str).unwrap();
    assert!(good_passport.valid());
}
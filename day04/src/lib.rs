use aoc2020::parse_newline_sep;

use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;
use thiserror::Error;

struct Passport{
    ecl: String,
    pid: String,
    eyr: String,
    hcl: String,
    byr: String,
    iyr: String,
    cid: String,
    hgt: String,
}

impl Passport {
    fn legal(&self) -> bool {
        [&self.ecl, &self.pid, &self.eyr, &self.hcl, &self.byr, &self.iyr, &self.hgt]
            .iter()
            .all(|field| *field != "")
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(input: &str) -> Result<Passport, Self::Err> {
        println!("{}", input);
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
            cid: passport_fields.get("cid").unwrap_or(&"").to_string(),
            hgt: passport_fields.get("hgt").unwrap_or(&"").to_string(),
        })
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let valid_count = parse_newline_sep::<Passport>(input)
                        ?.filter(|p| p.legal())
                        .count(); 
    println!("The answer to part one is {}", valid_count);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]

#[test]
fn test_valid_passport(){

}
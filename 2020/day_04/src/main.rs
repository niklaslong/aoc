#![feature(str_split_once)]

use std::io::{stdin, BufRead};

use regex::Regex;

fn main() {
    let stdin = stdin();
    let lines = stdin.lock().lines();

    let lines: Vec<String> = lines
        .into_iter()
        // Bad solution but in this case easy because data won't contain any negative values.
        .take_while(|line| line.as_ref().unwrap() != "-1")
        .map(|line| String::from(line.unwrap()))
        .collect();

    let candidates: Vec<PassportCandidate> = lines
        .split(|line| line.is_empty())
        .map(|line| line.join(" "))
        .map(|line| PassportCandidate::new(line))
        .collect();

    let valid = candidates
        .iter()
        .filter(|candidate| candidate.is_valid_passport())
        .count();

    println!("VALID: {}", valid);
}

#[derive(PartialEq, Debug, Default)]
struct PassportCandidate {
    // Birth Year.
    byr: Option<String>,
    // Issue Year.
    iyr: Option<String>,
    // Expiration Year.
    eyr: Option<String>,
    // Height.
    hgt: Option<String>,
    // Hair Color (should probably be Enum if values are known).
    hcl: Option<String>,
    // Eye Color.
    ecl: Option<String>,
    // Passport ID.
    pid: Option<String>,
    // Country ID.
    cid: Option<String>,
}

impl PassportCandidate {
    // probably not the best input param.
    fn new(l: String) -> Self {
        let mut candidate: PassportCandidate = Default::default();
        let elements: Vec<&str> = l.split(" ").collect();

        for e in elements {
            let (key, val) = e.split_once(":").unwrap();

            // Serde? Did this for the exercise but really should be using serde.
            match key {
                "byr" => candidate.byr = Some(String::from(val)),
                "iyr" => candidate.iyr = Some(String::from(val)),
                "eyr" => candidate.eyr = Some(String::from(val)),
                "hgt" => candidate.hgt = Some(String::from(val)),
                "hcl" => candidate.hcl = Some(String::from(val)),
                "ecl" => candidate.ecl = Some(String::from(val)),
                "pid" => candidate.pid = Some(String::from(val)),
                "cid" => candidate.cid = Some(String::from(val)),
                key => panic!("Invalid key: {}!", key),
            }
        }

        candidate
    }

    fn is_valid_passport(&self) -> bool {
        // Validate field present.
        let is_complete = self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();

        // Validate birth year.
        let mut byr_is_valid = false;

        if let Some(byr) = &self.byr {
            let byr: u32 = byr.parse().unwrap();

            byr_is_valid = 1920 <= byr && byr <= 2002;
        }

        // Validate issue year.
        let mut iyr_is_valid = false;

        if let Some(iyr) = &self.iyr {
            let iyr: u32 = iyr.parse().unwrap();

            iyr_is_valid = 2010 <= iyr && iyr <= 2020;
        }

        // Validate expiration year.
        let mut eyr_is_valid = false;

        if let Some(eyr) = &self.eyr {
            let eyr: u32 = eyr.parse().unwrap();

            eyr_is_valid = 2020 <= eyr && eyr <= 2030;
        }

        // Validate height.
        let mut hgt_is_valid = false;

        if let Some(hgt) = &self.hgt {
            if let Some(hgt) = hgt.strip_suffix("cm") {
                let hgt: u32 = hgt.parse().unwrap();

                hgt_is_valid = 150 <= hgt && hgt <= 193;
            }

            if let Some(hgt) = hgt.strip_suffix("in") {
                let hgt: u32 = hgt.parse().unwrap();

                hgt_is_valid = 59 <= hgt && hgt <= 76;
            }
        }

        // Validate hair colour.
        let mut hcl_is_valid = false;

        if let Some(hcl) = &self.hcl {
            let re = Regex::new(r"[[:alnum:]]").unwrap();

            hcl_is_valid = hcl.starts_with("#") && hcl.len() == 7 && re.is_match(hcl)
        }

        // Validate eye colour.
        let mut ecl_is_valid = false;

        if let Some(ecl) = &self.ecl {
            ecl_is_valid = match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            };
        }

        // Validate passport ID.
        let mut pid_is_valid = false;

        if let Some(pid) = &self.pid {
            pid_is_valid = pid.chars().count() == 9;
        }

        is_complete
            && byr_is_valid
            && iyr_is_valid
            && eyr_is_valid
            && hgt_is_valid
            && hcl_is_valid
            && ecl_is_valid
            && pid_is_valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_passport_candidate() {
        let input = String::from(
            "iyr:2010 ecl:gry hgt:181cm pid:591597745 byr:1920 hcl:#6b5442 eyr:2029 cid:123",
        );

        let candidate = PassportCandidate {
            byr: Some(String::from("1920")),
            iyr: Some(String::from("2010")),
            eyr: Some(String::from("2029")),
            hgt: Some(String::from("181cm")),
            hcl: Some(String::from("#6b5442")),
            ecl: Some(String::from("gry")),
            pid: Some(String::from("591597745")),
            cid: Some(String::from("123")),
        };

        assert_eq!(PassportCandidate::new(input), candidate);
    }

    #[test]
    fn all_fields_is_valid_passport() {
        let candidate = PassportCandidate {
            byr: Some(String::from("1920")),
            iyr: Some(String::from("2010")),
            eyr: Some(String::from("2025")),
            hgt: Some(String::from("181cm")),
            hcl: Some(String::from("#6b5442")),
            ecl: Some(String::from("gry")),
            pid: Some(String::from("591597745")),
            cid: Some(String::from("123")),
        };

        assert_eq!(candidate.is_valid_passport(), true);
    }

    #[test]
    fn missing_cid_is_valid_passport() {
        let candidate = PassportCandidate {
            byr: Some(String::from("1920")),
            iyr: Some(String::from("2010")),
            eyr: Some(String::from("2025")),
            hgt: Some(String::from("181cm")),
            hcl: Some(String::from("#6b5442")),
            ecl: Some(String::from("gry")),
            pid: Some(String::from("591597745")),
            cid: None,
        };

        assert_eq!(candidate.is_valid_passport(), true);
    }

    #[test]
    fn missing_field_is_invalid_passport() {
        // Note: except CID which is optional.

        let candidate = PassportCandidate {
            byr: None,
            iyr: Some(String::from("2010")),
            eyr: Some(String::from("2025")),
            hgt: Some(String::from("181")),
            hcl: Some(String::from("#6b5442")),
            ecl: Some(String::from("gry")),
            pid: Some(String::from("91597745")),
            cid: None,
        };

        assert_eq!(candidate.is_valid_passport(), false);
    }
}

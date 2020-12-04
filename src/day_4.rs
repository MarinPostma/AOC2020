use regex::Regex;
const INPUT: &str = include_str!("../inputs/day4.txt");


fn day4(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|pass| {
            pass.contains("byr") &&
                pass.contains("iyr") &&
                pass.contains("eyr") &&
                pass.contains("hgt") &&
                pass.contains("hcl") &&
                pass.contains("ecl") &&
                pass.contains("pid")
        })
    .count()
}

fn day4_part2(input: &str) -> usize {
    let extract_byr = Regex::new(r"byr:(\d{4})").unwrap();
    let extract_iyr = Regex::new(r"iyr:(\d{4})").unwrap();
    let extract_eyr = Regex::new(r"eyr:(\d{4})").unwrap();
    let extract_hgt = Regex::new(r"hgt:(\d+)(in|cm)").unwrap();
    let extract_hcl = Regex::new(r"hcl:#[0-9a-f]{6}").unwrap();
    let extract_ecl = Regex::new(r"ecl:amb|blu|brn|gry|grn|hzl|oth").unwrap();
    let extract_pid = Regex::new(r"pid:(\d+)").unwrap();
    input
        .split("\n\n")
        .filter(|pass| {
            let mut valid = true;

            valid &= extract_byr
                .captures(pass)
                .and_then(|c| c[1].parse::<usize>().ok())
                .map(|v| v >= 1920 && v <= 2002)
                .unwrap_or(false);

            valid &= extract_iyr
                .captures(pass)
                .and_then(|c| c[1].parse::<usize>().ok())
                .map(|v| v >= 2010 && v <= 2020)
                .unwrap_or(false);

            valid &= extract_eyr
                .captures(pass)
                .and_then(|c| c[1].parse::<usize>().ok())
                .map(|v| v >= 2020 && v <= 2030)
                .unwrap_or(false);

            valid &= extract_hgt
                .captures(pass)
                .and_then(|c| c[1].parse::<usize>().ok().map(move |v| (v, c.get(2).unwrap().as_str())))
                .and_then(|(h, m)| match m {
                    "cm" => Some(h >= 150 && h <= 193),
                    "in" => Some(h >= 59 && h <= 76),
                    _ => None,
                })
                .unwrap_or(false);

            //valid &= extract_hcl.is_match(pass);
            valid &= extract_hcl.is_match(pass);
            valid &= extract_ecl.is_match(pass);
            valid &= extract_pid
                .captures(pass)
                .map(|c| c[1].chars().count() == 9)
                .unwrap_or(false);

            valid
        })
    .count()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(day4(INPUT), 190);
        assert_eq!(day4_part2(INPUT), 121);
    }
}

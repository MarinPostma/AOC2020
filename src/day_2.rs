use regex::Regex;

const INPUT: &str = include_str!("../inputs/day2.txt");

fn day2(input: &str) -> usize{
    let re = Regex::new(r"^(\d+)-(\d+) (\w{1}): (\w*)$").unwrap();
    input
        .split("\n")
        .filter(|l| {
            if let Some(captures) = re.captures(l) {
                let lower: usize = captures[1].parse().unwrap();
                let higher: usize = captures[2].parse().unwrap();
                let rule = captures[3].chars().next().unwrap();
                let count = captures[4].chars().filter(|c| *c == rule).count();
                count >= lower && count <= higher
            } else {
                false
            }
        })
    .count()
}

fn day2_part2(input: &str) -> usize{
    let re = Regex::new(r"^(\d+)-(\d+) (\w{1}): (\w*)$").unwrap();
    input
        .split("\n")
        .filter(|l| {
            if let Some(captures) = re.captures(l) {
                let first: usize = captures[1].parse().unwrap();
                let second: usize = captures[2].parse().unwrap();
                let rule = captures[3].chars().next().unwrap();
                let mut chars = captures[4].chars();
                let ok_first = chars.nth(first - 1).map(|c| c  == rule).unwrap_or(false);
                let ok_second = chars.nth(second - first - 1).map(|c| c == rule).unwrap_or(false);
                (ok_second || ok_first) && !(ok_first && ok_second)
            } else {
                false
            }
        })
    .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(day2(INPUT), 569);
    }

    #[test]
    fn test2() {
        assert_eq!(day2_part2("1-3 a: abcde"), 1);
        assert_eq!(day2_part2(INPUT), 346);
    }
}

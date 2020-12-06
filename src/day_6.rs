use std::char;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day6.txt");

fn day_6(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| 
            s
                .chars()
                .filter(char::is_ascii_alphabetic)
                .sorted()
                .dedup()
                .count())
    .sum()
}

fn day_6_part2(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|s| s
            .lines()
            .map(|s| s.chars().fold(0u32, |set, c| set | 1 << c as u32 - 'a' as u32))
            .fold(None, |intersect, set| intersect.map(|s| s & set).or(Some(set)))
            .unwrap()
            .count_ones())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day6() {
        let test_input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(day_6(test_input), 11);
        assert_eq!(day_6(INPUT), 6335);
    }

    #[test]
    fn test_day6_part2() {
        let test_input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(day_6_part2(test_input), 6);
        assert_eq!(day_6_part2(INPUT), 3392);
    }
}

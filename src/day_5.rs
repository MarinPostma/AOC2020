use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day5.txt");

fn binary_search(seq: impl Iterator<Item = char>, end: usize, lower: char, upper: char) -> usize {
    let mut init = 0..end;
    for c in seq {
        let middle = init.start + (init.end - init.start) / 2;
        init = match c {
            c if c == upper => middle + 1..init.end,
            c if c == lower  => init.start..middle,
            _ => unreachable!(),
        };
    }
    init.start
}

fn get_seat_id(pass: &str) -> usize {
    let row = binary_search(pass.chars().take(7), 127, 'F', 'B');
    let col = binary_search(pass.chars().skip(7), 7, 'L', 'R');
    row * 8 + col
}

fn day5(input: &str) -> usize {
    input
        .lines()
        .map(get_seat_id)
        .max()
        .unwrap()
}

fn day5_part2(input: &str) -> usize {
    input
        .lines()
        .map(get_seat_id)
        .sorted()
        .tuple_windows()
        .find(|(a, b)| b - a == 2)
        .map(|(a, _)| a + 1)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day5() {
        assert_eq!(day5(INPUT), 816);
    }

    #[test]
    fn test_day5_part2() {
        assert_eq!(day5_part2(INPUT), 539);
    }
}

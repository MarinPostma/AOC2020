const INPUT: &str = include_str!("../inputs/day3.txt");

fn day_3(input: &str, y: usize, x: usize) -> usize{
    input
        .split_whitespace()
        .step_by(y)
        .skip(1)
        .enumerate()
        .filter(|(i, line)| {
            std::iter::repeat(line.chars())
                .flatten()
                .nth(x * (i + 1))
                .unwrap() == '#'
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        let map = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        assert_eq!(day_3(map, 1, 1), 2);
        assert_eq!(day_3(map, 1, 3), 7);
        assert_eq!(day_3(map, 1, 5), 3);
        assert_eq!(day_3(map, 1, 7), 4);
        assert_eq!(day_3(map, 2, 1), 2);

    }


    #[test]
    fn answer() {
        assert_eq!(day_3(INPUT, 1, 3), 225);
    }

    #[test]
    fn answer_part2() {
        assert_eq!(day_3(INPUT, 1, 1), 60);
        assert_eq!(day_3(INPUT, 1, 3), 225);
        assert_eq!(day_3(INPUT, 1, 5), 57);
        assert_eq!(day_3(INPUT, 1, 7), 58);
        assert_eq!(day_3(INPUT, 2, 1), 25);
        assert_eq!(60 * 225 * 57 * 58 * 25, 1115775000);
    }
}

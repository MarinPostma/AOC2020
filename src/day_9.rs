use std::cmp;

const INPUT: &str = include_str!("../inputs/day9.txt");

fn find_pair(vals: &[usize], value: usize) -> Option<(usize, usize)> {
    for (i, v) in vals.iter().enumerate() {
        if let Some(other) = vals[i..].iter().find(|other| *v + *other == value) {
            return Some((*v, *other))
        }
    }
    None
}

fn day_9(input: &str, preamble_size: usize) -> usize {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(preamble_size + 1)
        .find(|w| find_pair(&w[..preamble_size], w[preamble_size]).is_none())
        .map(|w| w[preamble_size])
        .unwrap()
}

fn day_9pt2(input: &str, value: usize) -> usize {
    let values = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();
    for i in 0..values.len() {
        let mut count = 0;
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for j in i..values.len() {
            let v = values[j];
            count += v;
            if count > value { break }
            min = cmp::min(min, v);
            max = cmp::max(max, v);
            if count == value { return max + min }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day9() {
        let sample = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
        assert_eq!(day_9(sample, 5), 127);
        assert_eq!(day_9(INPUT, 25), 1398413738);
    }

    #[test]
    fn test_day9pt2() {
        let sample = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
        assert_eq!(day_9pt2(sample, 127), 62);
        assert_eq!(day_9pt2(INPUT, 1398413738), 169521051);
    }
}

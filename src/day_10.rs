use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day10.txt");

fn day_10(input: &str) -> usize {
    let dist = std::iter::once(0)
        .chain(input.lines().map(|l| l.parse::<usize>().unwrap()))
        .sorted()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .fold([0, 0, 0, 1], |mut dist, diff| {
            dist[diff] += 1;
            dist
        });
    dist[1] * dist[3]
}

fn day_10_pt2(input: &str) -> usize {
    let mut max = usize::MIN;
    let input = std::iter::once(0)
        .chain(input.lines().map(|l| l.parse::<usize>().unwrap()))
        .sorted()
        .collect::<Vec<_>>();
    let mut graph = input
        .iter()
        .enumerate()
        .map(|(i, v)| {
            max = std::cmp::max(max, *v);
            (*v, input[i+1..].iter().cloned().take_while(|n| n - v <= 3).collect())
        })
        .collect::<HashMap<_, _>>();
    graph.insert(max, vec![max + 3]);
    graph.insert(max + 3, vec![]);
    let mut memoize = HashMap::new();
    count_path(0, max + 3, &graph, &mut memoize)
}

fn count_path(
    from: usize,
    to: usize,
    graph: &HashMap<usize, Vec<usize>>,
    memoize: &mut HashMap<(usize, usize), usize>
    ) -> usize {
    if from == to {
        1
    } else {
        let current = &graph[&from];
        let mut count = 0;
        for from in current {
            count += match memoize.get(&(*from, to)) {
                Some(count) => *count,
                None => {
                    let count = count_path(*from, to, graph, memoize);
                    memoize.insert((*from, to), count);
                    count
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pt2() {
        let sample1 = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        assert_eq!(day_10_pt2(sample1), 8);
        let sample2 = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
        assert_eq!(day_10_pt2(sample2), 19208);
        assert_eq!(day_10_pt2(INPUT), 148098383347712);
    }

    #[test]
    fn test() {
        let sample1 = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        assert_eq!(day_10(sample1), 35);
        let sample2 = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
        assert_eq!(day_10(sample2), 220);
        assert_eq!(day_10(INPUT), 2664);
    }
}

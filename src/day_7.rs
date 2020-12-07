use regex::Regex;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../inputs/day7.txt");

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<amount>\d)*\s?(?P<name>[a-z\s]+) bags?[,|.]?").unwrap());

/// returns an iterator over (container, contained, amount)
fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input.lines().fold(HashMap::new(), |mut map, line| {
        let mut split = line.split(" bags contain ");
        let container = split.next().unwrap();
        RE.captures_iter(split.next().unwrap())
            .filter_map(|c| (&c["name"] != "no other").then(|| c["name"].to_owned()))
            .for_each(|containee| map.entry(containee).or_default().push(container.to_owned()));
        map
    })
}

fn parse_input2(input: &str) -> HashMap<String, Vec<(String, usize)>> {
    input.lines().fold(HashMap::new(), |mut map, line| {
        let mut split = line.split(" bags contain ");
        let container = split.next().unwrap();
        let containees = RE
            .captures_iter(split.next().unwrap())
            .filter_map(|c| (&c["name"] != "no other").then(|| (c["name"].to_owned(), c["amount"].parse::<usize>().unwrap())))
            .collect();
        map.insert(container.to_owned(), containees);
        map
    })
}

fn get_visit_recur(graph: &HashMap<String, Vec<String>>, bag: &str, visited: &mut HashSet<String>) -> usize {
    visited.insert(bag.to_owned());
    match graph.get(bag) {
        None => 1,
        Some(bags) => 1 + bags
            .iter()
            .filter_map(|bag| (!visited.contains(bag)).then(|| get_visit_recur(graph, bag, visited)))
            .sum::<usize>()
    }
}

fn day7(input: &'static str,) -> usize {
    let graph = parse_input(input);
    let mut visited = HashSet::new();
    get_visit_recur(&graph, "shiny gold", &mut visited) - 1
}

fn get_count_recur(graph: &HashMap<String, Vec<(String, usize)>>, bag: &str) -> usize {
    graph
        .get(bag)
        .map(|bags|
            bags
            .iter()
            .map(|(bag, amount)| amount + amount * get_count_recur(graph, bag))
            .sum::<usize>()
        ).unwrap_or_default()
}

fn day7_pt2(input: &'static str,) -> usize {
    let graph = parse_input2(input);
    get_count_recur(&graph, "shiny gold")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day7() {
        let sample = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        assert_eq!(day7(sample), 4);
        assert_eq!(day7(INPUT), 348);
        let sample2 = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        assert_eq!(day7_pt2(sample2), 126);
        assert_eq!(day7_pt2(sample), 32);
        assert_eq!(day7_pt2(INPUT), 18885);
    }
}

use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day8.txt");

fn parse_pgm(input: &str) -> impl Iterator<Item=(&str, isize)> {
    input
        .lines()
        .map(|s| {
            let (inst, val) = s.split_at(3);
            (inst, val.trim().parse().unwrap())
        })
}

fn day8(input: &str) -> isize {
    let pgm: Vec<_> = parse_pgm(input).collect();
    let mut pc = 0isize;
    let mut counter = 0;
    let mut visited = HashSet::new();
    loop {
        let (inst, val) = pgm[pc as usize];
        if visited.contains(&pc) { break }
        visited.insert(pc);
        match inst {
            "nop" => { pc += 1; },
            "acc" => {
                pc += 1;
                counter += val;
            }
            "jmp" => { pc += val; }
            _ => ()
        }
    }
    counter
}

fn day8_pt2(input: &str) -> Option<isize> {
    let pgm: Vec<_> = parse_pgm(input).collect();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut pc = 0;

    while (pc as usize) < pgm.len() {
        let (inst, val) = pgm[pc as usize];
        if visited.contains(&pc) { break }
        if inst == "nop" || inst == "jmp" { stack.push(pc) }
        visited.insert(pc);
        match inst {
            "jmp" => { pc += val; }
            _ => { pc += 1; },
        }
    }
    'outer: for candidate in stack.into_iter().rev() {
        let mut counter = 0;
        visited.clear();
        pc = 0;
        while (pc as usize) < pgm.len() {
            if visited.contains(&pc) { continue 'outer; }
            visited.insert(pc);
            let (inst, val) = match pgm[pc as usize] {
                ("nop", val) if pc == candidate => ("jmp", val),
                ("jmp", val) if pc == candidate => ("nop", val),
                (inst, val) => (inst, val),
            };
            match inst {
                "nop" => { pc += 1; },
                "jmp" => { pc += val; }
                "acc" => { pc += 1; counter += val; }
                _ => (),
            }
        }
        return Some(counter)
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day8() {
        let sample = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        assert_eq!(day8(sample), 5);
        assert_eq!(day8(INPUT), 2025);
    }

    #[test]
    fn test_day8_pt2() {
        let sample = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        assert_eq!(day8_pt2(sample), Some(8));
        assert_eq!(day8_pt2(INPUT), Some(2001));
    }
}

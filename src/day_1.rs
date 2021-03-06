use rdxsort::*;
use voracious_radix_sort::{RadixSort};
use std::cmp::Ordering;
use std::collections::HashSet;
use vec_map::VecMap;

fn find_pair(vals: &mut [usize]) -> Option<(usize, usize)> {
    vals.sort();
    let mut first = 0;
    let mut second = vals.len() - 1;
    while first <= second {
        let sum = vals[first] + vals[second];
        match 2020.cmp(&sum) {
            Ordering::Less => {
                second -= 1;
            }
            Ordering::Equal => return Some((vals[first], vals[second])),
            Ordering::Greater => {
                first += 1;
            }
        }
    }
    None
}

fn find_pair_unstable(vals: &mut [usize]) -> Option<(usize, usize)> {
    vals.sort_unstable();
    let mut first = 0;
    let mut second = vals.len() - 1;
    while first <= second {
        let sum = vals[first] + vals[second];
        match 2020.cmp(&sum) {
            Ordering::Less => {
                second -= 1;
            }
            Ordering::Equal => return Some((vals[first], vals[second])),
            Ordering::Greater => {
                first += 1;
            }
        }
    }
    None
}

fn find_pair_rdx(vals: &mut [usize]) -> Option<(usize, usize)> {
    vals.rdxsort();
    let mut first = 0;
    let mut second = vals.len() - 1;
    while first <= second {
        let sum = vals[first] + vals[second];
        match 2020.cmp(&sum) {
            Ordering::Less => {
                second -= 1;
            }
            Ordering::Equal => return Some((vals[first], vals[second])),
            Ordering::Greater => {
                first += 1;
            }
        }
    }
    None
}

fn find_pair_voracious(vals: &mut [usize]) -> Option<(usize, usize)> {
    vals.voracious_sort();
    let mut first = 0;
    let mut second = vals.len() - 1;
    while first <= second {
        let sum = vals[first] + vals[second];
        match 2020.cmp(&sum) {
            Ordering::Less => {
                second -= 1;
            }
            Ordering::Equal => return Some((vals[first], vals[second])),
            Ordering::Greater => {
                first += 1;
            }
        }
    }
    None
}

fn find_pair_hashset(vals: &mut [usize]) -> Option<(usize, usize)> {
    let set = vals.iter().cloned().collect::<HashSet<_>>();
    for v in vals {
        if set.contains(&(2020 - *v)) {
            return Some((*v, 2020 - *v))
        }
    }
    None
}

fn find_pair_vecmap(vals: &[usize]) -> Option<(usize, usize)> {
    let set = vals.iter().map(|v| (*v, ())).collect::<VecMap<_>>();
    for v in vals {
        if let Some(_) = set.get(2020 - v) {
            return Some((*v, 2020 - v))
        }
    }
    None
}

fn find_pair_bruteforce(vals: &[usize]) -> Option<(usize, usize)> {
    for (i, v) in vals.iter().enumerate() {
        if let Some(other) = vals[i..].iter().find(|other| *v + *other == 2020) {
            return Some((*v, *other))
        }
    }
    None
}

fn find_pair_bruteforce_sorted(vals: &mut [usize]) -> Option<(usize, usize)> {
    vals.sort_unstable();
    for first in 0..vals.len() {
        let first_val = vals[first];
        'second: for second in first + 1..vals.len() {
            let second_val = vals[second];
            let sum = first_val + second_val;
            if sum > 2020 {
                break 'second;
            } else if sum == 2020 {
                return Some((first_val, second_val));
            }
        }
    }
    None
}

fn part_two(vals: &mut [usize]) -> Option<(usize, usize, usize)> {
    vals.sort_unstable();
    for first in 0..vals.len() {
        let first_val = vals[first];
        for second in first + 1..vals.len() {
            let second_val = vals[second];
            'third: for third in second + 1..vals.len() {
                let third_val = vals[third];
                let sum = first_val + second_val + third_val;
                if sum > 2020 {
                    break 'third;
                } else if sum == 2020 {
                    return Some((first_val, second_val, third_val));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use ::test::Bencher;

    const INPUT: &[usize] = &[1753, 1858, 1860, 1978, 1758, 1847, 2010, 1679, 1222, 1723, 1592, 1992, 1865, 1635, 1692, 1653, 1485, 848, 1301, 1818, 1872, 1883, 1464, 2002, 1736, 1821, 1851, 1299, 1627, 1698, 1713, 1676, 1673, 1448, 1939, 1506, 1896, 1710, 1677, 1894, 1645, 1454, 1972, 1687, 265, 1923, 1666, 1761, 1386, 2006, 1463, 1759, 1460, 1722, 1670, 1731, 1732, 1976, 1564, 1380, 1981, 1998, 1912, 1479, 1500, 167, 1904, 1689, 1810, 1675, 1811, 1671, 1535, 1624, 1638, 1848, 1646, 1795, 1717, 1803, 1867, 1794, 1774, 1245, 1915, 1601, 1656, 1472, 1700, 1887, 1869, 1876, 1561, 1743, 1900, 1574, 1400, 1950, 1893, 1576, 1903, 1747, 1560, 1445, 1652, 633, 1970, 1812, 1807, 1788, 1948, 1588, 1639, 1719, 1680, 1773, 1890, 1347, 1344, 1456, 1691, 1842, 1585, 1953, 410, 1791, 485, 1412, 1994, 1799, 1955, 1554, 1661, 1708, 1824, 1553, 1993, 1911, 1515, 1545, 856, 1685, 1982, 1954, 1480, 1709, 1428, 1829, 1606, 1613, 1941, 1483, 1513, 1664, 1801, 1720, 1984, 1575, 1805, 1833, 1418, 1882, 1746, 483, 1674, 1467, 1453, 523, 1414, 1800, 1403, 1946, 1868, 1520, 1861, 1580, 1995, 1960, 1625, 1411, 1558, 1817, 1854, 1617, 1478, 735, 1593, 1778, 1809, 1584, 1438, 1845, 1712, 1655, 1990, 1578, 1703, 1895, 1765, 1572, ];

    #[test]
    fn test_sort() {
        let mut input = Vec::from(INPUT);
        assert_eq!(find_pair(&mut input), Some((485, 1535)));
    }

    macro_rules! make_bench {
        ($fn_name:ident, $name:ident) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut input = Vec::from(INPUT);
                b.iter(|| $fn_name(&mut input));
            }
        };
    }

    make_bench!(find_pair_bruteforce, bench_bruteforce);
    make_bench!(find_pair_vecmap, bench_vecmap);
    make_bench!(find_pair_hashset, bench_hashset);
    make_bench!(find_pair, bench_sort);
    make_bench!(find_pair_unstable, bench_sort_unstable);
    make_bench!(find_pair_rdx, bench_sort_rdx);
    make_bench!(find_pair_voracious, bench_sort_voracious);
    make_bench!(part_two, bench_part_two);
    make_bench!(find_pair_bruteforce_sorted, bench_part_bruteforce_sorted);

    //test day_1::test::bench_bruteforce             ... bench:       5,849 ns/iter (+/- 123)
    //test day_1::test::bench_hashset                ... bench:       5,082 ns/iter (+/- 739)
    //test day_1::test::bench_part_bruteforce_sorted ... bench:         370 ns/iter (+/- 0)
    //test day_1::test::bench_part_two               ... bench:         154 ns/iter (+/- 1)
    //test day_1::test::bench_sort                   ... bench:         362 ns/iter (+/- 12)
    //test day_1::test::bench_sort_rdx               ... bench:       8,744 ns/iter (+/- 293)
    //test day_1::test::bench_sort_unstable          ... bench:         311 ns/iter (+/- 10)
    //test day_1::test::bench_sort_voracious         ... bench:         313 ns/iter (+/- 126)
    //test day_1::test::bench_vecmap                 ... bench:         527 ns/iter (+/- 18)


    #[test]
    fn test_bruteforce() {
        let mut input = Vec::from(INPUT);
        assert_eq!(find_pair_bruteforce(&mut input), Some((1535, 485)));
    }

    #[test]
    fn test_hashset() {
        let mut input = Vec::from(INPUT);
        assert_eq!(find_pair_hashset(&mut input), Some((1535, 485)));
    }

    #[test]
    fn test_vecmap() {
        let mut input = Vec::from(INPUT);
        assert_eq!(find_pair_vecmap(&mut input), Some((1535, 485)));
    }

    #[test]
    fn test_part2() {
        let mut input = Vec::from(INPUT);
        assert_eq!(part_two(&mut input), Some((167, 265, 1588)));
    }
}

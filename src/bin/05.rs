use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut nums: Vec<u64> = lines.next().unwrap()[7..]
        .split(' ')
        .map(|val| val.parse().unwrap())
        .collect();

    let mut map_list: Vec<(u64, u64, u64)> = Vec::new();
    loop {
        let Some(line) = lines.next() else {
            'a: for num in nums.iter_mut() {
                for maps in &map_list {
                    if *num >= maps.1 && *num < maps.1 + maps.2 {
                        *num = *num + maps.0 - maps.1;
                        continue 'a;
                    }
                }
            }
            break;
        };

        if line.is_empty() {
            'a: for num in nums.iter_mut() {
                for maps in &map_list {
                    if *num >= maps.1 && *num < maps.1 + maps.2 {
                        *num = *num + maps.0 - maps.1;
                        continue 'a;
                    }
                }
            }

            lines.next();
            map_list.clear();
            continue;
        }

        map_list.push(
            line.split(' ')
                .map(|val| val.parse().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }

    Some(*nums.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let mut ranges: VecDeque<(u64, u64)> = lines.next().unwrap()[7..]
        .split(' ')
        .map(|val| val.parse().unwrap())
        .tuples()
        .collect();

    let mut map_list: Vec<(u64, u64, u64)> = Vec::new();
    loop {
        let Some(line) = lines.next() else {
            // 'a: for num in nums.iter_mut() {
            //     for maps in &map_list {
            //         if *num >= maps.1 && *num < maps.1 + maps.2 {
            //             *num = *num + maps.0 - maps.1;
            //             continue 'a;
            //         }
            //     }
            // }
            break;
        };

        if line.is_empty() {
            let mut out_ranges: VecDeque<(u64, u64)> = VecDeque::new();

            for maps in &map_list {
                let orig_len = ranges.len();
                for _ in 0..orig_len {
                    let range = ranges.pop_front().unwrap();

                    if maps.1 < range.0 && maps.1 + maps.2 < range.0 + range.1 {
                        // WRONG
                        out_ranges.push_back((range.0 + maps.0 - maps.1, maps.2));
                        ranges.push_back((maps.0 + maps.1, range.0 + range.1 - maps.0 - maps.1));
                    } else if maps.1 < range.0 && maps.1 + maps.2 >= range.0 + range.1 {
                        out_ranges.push_back((maps.0, maps.2));
                    } else if maps.1 >= range.0 && maps.1 + maps.2 < range.0 + range.1 {
                    } else if maps.1 >= range.0 && maps.1 + maps.2 >= range.0 + range.1 {
                    } else {
                    }
                }
            }
            // 'a: for range in ranges.iter_mut() {
            //     for maps in &map_list {
            //         if *num >= maps.1 && *num < maps.1 + maps.2 {
            //             *num = *num + maps.0 - maps.1;
            //             continue 'a;
            //         }
            //     }
            // }

            lines.next();
            map_list.clear();
            continue;
        }

        map_list.push(
            line.split(' ')
                .map(|val| val.parse().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

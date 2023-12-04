advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|line| Vec::from(line.as_bytes()))
        .collect();

    let dim = (input.len(), input[0].len());

    let mut flags: Vec<Vec<bool>> = vec![vec![false; dim.1]; dim.0];

    for (y, line) in input.iter().enumerate() {
        for (x, &ele) in line.iter().enumerate() {
            // is symbol
            if ele != b'.' && !ele.is_ascii_digit() {
                #[allow(clippy::needless_range_loop)]
                for s_y in usize::max(y, 1) - 1..=usize::min(y + 1, dim.0 - 1) {
                    for s_x in usize::max(x, 1) - 1..=usize::min(x + 1, dim.1 - 1) {
                        flags[s_y][s_x] = true;
                    }
                }
            }
        }
    }

    let mut count = 0;

    for (y, line) in input.iter().enumerate() {
        let mut x = 0;
        'a: while x < dim.1 {
            let ele = line[x];

            // is number
            if ele.is_ascii_digit() {
                let mut number: u32 = 0;
                let mut is_near = false;

                let mut s_x = x;
                loop {
                    number = number * 10 + (line[s_x] - b'0') as u32;

                    if flags[y][s_x] {
                        is_near = true;
                    }

                    s_x += 1;
                    if s_x == dim.1 || !line[s_x].is_ascii_digit() {
                        // END
                        if is_near {
                            count += number;
                        }
                        x = s_x;
                        continue 'a;
                    }
                }
            }

            x += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|line| Vec::from(line.as_bytes()))
        .collect();

    let dim = (input.len(), input[0].len());

    let mut count = 0;

    let mut nums: Vec<Vec<u32>> = vec![vec![0; dim.1]; dim.0];

    for (y, line) in input.iter().enumerate() {
        let mut x = 0;
        'a: while x < dim.1 {
            let ele = line[x];

            // is number
            if ele.is_ascii_digit() {
                let mut number: u32 = 0;

                let mut s_x = x;
                loop {
                    number = number * 10 + (line[s_x] - b'0') as u32;

                    s_x += 1;
                    if s_x == dim.1 || !line[s_x].is_ascii_digit() {
                        // END
                        nums[y][x..s_x].fill(number);
                        x = s_x;
                        continue 'a;
                    }
                }
            }

            x += 1;
        }
    }

    for (y, line) in input.iter().enumerate() {
        for (x, &ele) in line.iter().enumerate() {
            // is gear
            if ele == b'*' {
                let mut num_count = 0;
                let mut ratio = 1;
                #[allow(clippy::needless_range_loop)]
                for s_y in usize::max(y, 1) - 1..=usize::min(y + 1, dim.0 - 1) {
                    for s_x in usize::max(x, 1) - 1..=usize::min(x + 1, dim.1 - 1) {
                        if nums[s_y][s_x] != 0 && (s_x == x || nums[s_y][x] == 0) {
                            num_count += 1;
                            ratio *= nums[s_y][s_x];
                        }
                    }
                }

                if num_count == 2 {
                    count += ratio;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}

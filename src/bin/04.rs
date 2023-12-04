advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap().1.split_once('|').unwrap();

            let winning_nums: Vec<u32> = left
                .split_ascii_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();
            let nums = right
                .split_ascii_whitespace()
                .map(|val| val.parse::<u32>().unwrap());

            let mut count = 0;
            for num in nums {
                if winning_nums.contains(&num) {
                    if count == 0 {
                        count = 1;
                    } else {
                        count *= 2;
                    }
                }
            }

            count
        })
        .sum();

    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<&str> = input.lines().collect();
    let mut cards: Vec<u32> = vec![1; input.len()];

    for (i, line) in input.iter().enumerate() {
        let (left, right) = line.split_once(':').unwrap().1.split_once('|').unwrap();

        let winning_nums: Vec<u32> = left
            .split_ascii_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();
        let nums = right
            .split_ascii_whitespace()
            .map(|val| val.parse::<u32>().unwrap());

        let this_count = cards[i];
        let count = nums.filter(|num| winning_nums.contains(num)).count();

        cards[i + 1..i + 1 + count]
            .iter_mut()
            .for_each(|val| *val += this_count);
    }

    Some(cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut nums = line.bytes().filter(|c| c.is_ascii_digit()).peekable();
                ((nums.peek().unwrap() - b'0') * 10 + (nums.last().unwrap() - b'0')) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // This is very slow, I know...
    let new_input = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "f4ur")
        .replace("five", "f5ve")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "n9ne");

    part_one(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}

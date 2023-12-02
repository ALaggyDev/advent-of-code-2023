advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();

        let game_num: u32 = left.split_once(' ').unwrap().1.parse().unwrap();

        let mut possible = true;
        for round in right.split("; ") {
            for ele in round.split(", ") {
                let (num, color) = ele.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();

                if (color == "red" && num > 12)
                    || (color == "green" && num > 13)
                    || (color == "blue" && num > 14)
                {
                    possible = false;
                }
            }
        }

        if possible {
            count += game_num;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let (_, right) = line.split_once(": ").unwrap();

        let mut mins = [0; 3];
        for round in right.split("; ") {
            for ele in round.split(", ") {
                let (num, color) = ele.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();

                match color {
                    "red" => {
                        if mins[0] < num {
                            mins[0] = num;
                        }
                    }
                    "green" => {
                        if mins[1] < num {
                            mins[1] = num;
                        }
                    }
                    "blue" => {
                        if mins[2] < num {
                            mins[2] = num;
                        }
                    }
                    _ => {}
                }
            }
        }

        count += mins[0] * mins[1] * mins[2];
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

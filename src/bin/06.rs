advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let time: Vec<i32> = lines.next().unwrap()[5..]
        .split_ascii_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();
    let distance: Vec<i32> = lines.next().unwrap()[9..]
        .split_ascii_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();

    let mut count = 1;
    for i in 0..time.len() {
        // (max_time - t) * t > distance
        // -t^2 + max_time*t - distance > 0
        let a = -1.0;
        let b = time[i] as f32;
        let c = -distance[i] as f32;

        // I could do some fancy algebra, but the floor and ceil makes this really hard to do correctly
        let low = ((-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)).floor() as u32;
        let high = ((-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)).ceil() as u32;

        count *= high - low - 1;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let time: i64 = lines.next().unwrap()[5..].replace(' ', "").parse().unwrap();
    let distance: i64 = lines.next().unwrap()[9..].replace(' ', "").parse().unwrap();

    let a = -1.0;
    let b = time as f64;
    let c = -distance as f64;

    let low = ((-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)).floor() as u32;
    let high = ((-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)).ceil() as u32;

    Some(high - low - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}

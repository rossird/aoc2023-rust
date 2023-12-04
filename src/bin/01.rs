advent_of_code::solution!(1);

const RADIX: u32 = 10;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0_u32, |acc, l| {
        let mut v = vec![];
        for c in l.chars() {
            if let Some(digit) = c.to_digit(RADIX) {
                v.push(digit);
            }
        }
        acc + v.first().unwrap() * 10 + v.last().unwrap()
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    const RADIX: u32 = 10;
    let mut s = 0;
    input.lines().for_each(|l| {
        let mut v = vec![];
        let mut iter = l.chars().enumerate();
        while let Some((i, c)) = iter.next() {
            if let Some(digit) = c.to_digit(RADIX) {
                v.push(digit);
                continue;
            }
            for (str, val) in [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ] {
                if l[i..].starts_with(str) {
                    v.push(val);
                    break;
                }
            }
        }
        s += v.first().unwrap() * 10 + v.last().unwrap();
    });
    Some(s)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}

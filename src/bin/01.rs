advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    const RADIX: u32 = 10;
    let mut s = 0;
    input.lines().for_each(|l| {
        let mut v = vec![];
        for c in l.chars() {
            if c.is_digit(RADIX) {
                v.push(c.to_digit(RADIX).unwrap());
            }
        }
        s += v[0] * 10 + v.last().unwrap();
    });
    Some(s)
}

pub fn part_two(input: &str) -> Option<u32> {
    const RADIX: u32 = 10;
    let mut s = 0;
    input.lines().for_each(|l| {
        let mut v = vec![];
        for (i, c) in l.chars().enumerate() {
            if c.is_digit(RADIX) {
                v.push(c.to_digit(RADIX).unwrap());
            }
            if l.chars().skip(i).take(3).collect::<String>() == "one" {
                v.push(1);
            }
            if l.chars().skip(i).take(3).collect::<String>() == "two" {
                v.push(2);
            }
            if l.chars().skip(i).take(5).collect::<String>() == "three" {
                v.push(3);
            }
            if l.chars().skip(i).take(4).collect::<String>() == "four" {
                v.push(4);
            }
            if l.chars().skip(i).take(4).collect::<String>() == "five" {
                v.push(5);
            }
            if l.chars().skip(i).take(3).collect::<String>() == "six" {
                v.push(6);
            }
            if l.chars().skip(i).take(5).collect::<String>() == "seven" {
                v.push(7);
            }
            if l.chars().skip(i).take(5).collect::<String>() == "eight" {
                v.push(8);
            }
            if l.chars().skip(i).take(4).collect::<String>() == "nine" {
                v.push(9);
            }
        }
        s += v[0] * 10 + v.last().unwrap();
    });
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

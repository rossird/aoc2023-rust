advent_of_code::solution!(2);
#[derive(Default, Debug)]
struct Set {
    green: u32,
    red: u32,
    blue: u32,
}

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        let mut s: Set = Default::default();
        for c in value.split(',').map(|s| s.trim()) {
            let parts = c.split_whitespace().collect::<Vec<_>>();
            if parts.len() != 2 {
                continue;
            }

            let count = parts[0].parse::<u32>().unwrap_or(0);
            match parts[1] {
                "red" => s.red = count,
                "blue" => s.blue = count,
                "green" => s.green = count,
                _ => (),
            }
        }
        s
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut r = 0u32;
    input.lines().for_each(|l| {
        let parts: Vec<_> = l.split(":").collect();
        if parts.len() != 2 {
            return;
        }
        let n = parts[0]
            .chars()
            .skip(5) // "Game "
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or(0);
        parts[1]
            .split(";")
            .map(|s| Set::from(s.trim()))
            .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
            .then(|| r += n);
    });
    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut r = 0u32;
    input.lines().for_each(|l| {
        let mut min_s: Set = Default::default();
        let parts = l.split(":").collect::<Vec<_>>();
        if parts.len() != 2 {
            return;
        }

        parts[1]
            .split(";")
            .map(|s| Set::from(s.trim()))
            .for_each(|s| {
                min_s.red = std::cmp::max(min_s.red, s.red);
                min_s.blue = std::cmp::max(min_s.blue, s.blue);
                min_s.green = std::cmp::max(min_s.green, s.green);
            });
        r += min_s.red * min_s.blue * min_s.green;
    });
    Some(r)
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

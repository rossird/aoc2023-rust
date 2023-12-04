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
        for c in value.split(",") {
            let parts = c.trim().split(" ").collect::<Vec<_>>();
            let n = parts[0].parse::<u32>().unwrap();
            let color = parts[1];
            match color {
                "red" => s.red = n,
                "blue" => s.blue = n,
                "green" => s.green = n,
                _ => println!("unrecognized color: {}", color),
            }
        }
        return s;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut r = 0u32;
    input.lines().for_each(|l| {
        let parts = l.split(":").collect::<Vec<_>>();
        let n = parts[0]
            .chars()
            .skip(5)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let sets = parts[1]
            .split(";")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|s| Set::from(s.trim()))
            .collect::<Vec<Set>>();
        if sets
            .iter()
            .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
        {
            r += n;
        }
    });
    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut r = 0u32;
    input.lines().for_each(|l| {
        let mut min_s: Set = Default::default();
        let parts = l.split(":").collect::<Vec<_>>();
        let n = parts[0]
            .chars()
            .skip(5)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let sets = parts[1]
            .split(";")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|s| Set::from(s.trim()))
            .collect::<Vec<Set>>();
        for s in sets {
            min_s.red = std::cmp::max(min_s.red, s.red);
            min_s.blue = std::cmp::max(min_s.blue, s.blue);
            min_s.green = std::cmp::max(min_s.green, s.green);
        }
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

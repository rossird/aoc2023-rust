use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut r = 0;
    input.lines().for_each(|l| {
        let parts: Vec<_> = l.split(':').collect();
        let parts: Vec<_> = parts[1].split('|').collect();

        let winning_nums: Vec<_> = parts[0]
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let winning_nums: HashSet<u32> = HashSet::from_iter(winning_nums.into_iter());

        let my_nums: Vec<_> = parts[1]
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let my_nums: HashSet<u32> = HashSet::from_iter(my_nums.into_iter());

        let intersection = winning_nums.intersection(&my_nums);
        let len = TryInto::<u32>::try_into(intersection.clone().collect::<Vec<_>>().len()).unwrap();
        let points = if len > 0 { 2_u32.pow(len - 1) } else { 0 };
        r += points;
    });
    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut instances = HashMap::<usize, u32>::new();
    input.lines().enumerate().for_each(|(i, l)| {
        let v = instances.entry(i).or_insert(0);
        *v += 1;

        let parts: Vec<_> = l.split(':').collect();
        let parts: Vec<_> = parts[1].split('|').collect();

        let winning_nums: Vec<_> = parts[0]
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let winning_nums: HashSet<u32> = HashSet::from_iter(winning_nums.into_iter());

        let my_nums: Vec<_> = parts[1]
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let my_nums: HashSet<u32> = HashSet::from_iter(my_nums.into_iter());

        let intersection = winning_nums.intersection(&my_nums);
        let len = intersection.collect::<Vec<_>>().len();
        if len == 0 {
            return;
        }

        let copies = *instances.get(&i).unwrap_or(&1);
        for x in i + 1..(i + 1 + len) {
            let v = instances.entry(x).or_insert(0);
            *v += copies;
        }
    });
    Some(instances.values().sum())
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

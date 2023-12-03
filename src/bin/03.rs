use std::collections::HashSet;

advent_of_code::solution!(3);

const RADIX: u32 = 10;

pub fn part_one(input: &str) -> Option<u32> {
    // character is a symbol if its not a digit or a period
    let is_symbol = |c: char| !(c.is_digit(RADIX) || c == '.');

    // check if index is a number.
    // if it is, return the number and the indices used
    // otherwise, return None
    let check_index =
        |i: usize, j: usize, schematic: &Vec<Vec<char>>, checked: &HashSet<(usize, usize)>| {
            if checked.contains(&(i, j)) || !schematic[i][j].is_digit(RADIX) {
                return None;
            }

            let mut checked = HashSet::new();
            checked.insert((i, j));
            let mut start = j;
            while start > 0 && schematic[i][start - 1].is_digit(RADIX) {
                start -= 1;
                checked.insert((i, start));
            }
            let mut end = j;
            while end < schematic[i].len() - 1 && schematic[i][end + 1].is_digit(RADIX) {
                end += 1;
                checked.insert((i, end));
            }
            let num = schematic[i][start..=end]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            Some((num, checked))
        };

    //let mut numbers = std::collections::HashMap::new();

    let mut schematic: Vec<Vec<char>> = Vec::new();
    input
        .lines()
        .for_each(|l| schematic.push(l.chars().collect()));
    let num_lines = schematic.len();

    let mut r = 0;

    for (i, line) in schematic.iter().enumerate() {
        let line_len = line.len();
        for (j, c) in line.iter().enumerate() {
            if is_symbol(*c) {
                let mut indices_checked = std::collections::HashSet::new();
                if i > 0 {
                    if j > 0 {
                        match check_index(i - 1, j - 1, &schematic, &indices_checked) {
                            Some((n, indices)) => {
                                r += n;
                                indices_checked.extend(indices);
                            }
                            None => (),
                        }
                    }
                    match check_index(i - 1, j, &schematic, &indices_checked) {
                        Some((n, indices)) => {
                            r += n;
                            indices_checked.extend(indices);
                        }
                        None => (),
                    }
                    if j < line_len - 1 {
                        match check_index(i - 1, j + 1, &schematic, &indices_checked) {
                            Some((n, indices)) => {
                                r += n;
                                indices_checked.extend(indices);
                            }
                            None => (),
                        }
                    }
                }

                if j > 0 {
                    match check_index(i, j - 1, &schematic, &indices_checked) {
                        Some((n, indices)) => {
                            r += n;
                            indices_checked.extend(indices);
                        }
                        None => (),
                    }
                }
                if j < line_len - 1 {
                    match check_index(i, j + 1, &schematic, &indices_checked) {
                        Some((n, indices)) => {
                            r += n;
                            indices_checked.extend(indices);
                        }
                        None => (),
                    }
                }

                if i < num_lines - 1 {
                    if j > 0 {
                        match check_index(i + 1, j - 1, &schematic, &indices_checked) {
                            Some((n, indices)) => {
                                r += n;
                                indices_checked.extend(indices);
                            }
                            None => (),
                        }
                    }
                    match check_index(i + 1, j, &schematic, &indices_checked) {
                        Some((n, indices)) => {
                            r += n;
                            indices_checked.extend(indices);
                        }
                        None => (),
                    }
                    if j < line_len - 1 {
                        match check_index(i + 1, j + 1, &schematic, &indices_checked) {
                            Some((n, indices)) => {
                                r += n;
                                indices_checked.extend(indices);
                            }
                            None => (),
                        }
                    }
                }
            }
        }
    }

    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    // check if index is a number.
    // if it is, return the number and the indices used
    // otherwise, return None
    let check_index =
        |i: usize, j: usize, schematic: &Vec<Vec<char>>, checked: &HashSet<(usize, usize)>| {
            if checked.contains(&(i, j)) || !schematic[i][j].is_digit(RADIX) {
                return None;
            }

            let mut checked = HashSet::new();
            checked.insert((i, j));
            let mut start = j;
            while start > 0 && schematic[i][start - 1].is_digit(RADIX) {
                start -= 1;
                checked.insert((i, start));
            }
            let mut end = j;
            while end < schematic[i].len() - 1 && schematic[i][end + 1].is_digit(RADIX) {
                end += 1;
                checked.insert((i, end));
            }
            let num = schematic[i][start..=end]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            Some((num, checked))
        };

    //let mut numbers = std::collections::HashMap::new();

    let mut schematic: Vec<Vec<char>> = Vec::new();
    input
        .lines()
        .for_each(|l| schematic.push(l.chars().collect()));
    let num_lines = schematic.len();

    let mut r = 0;

    for (i, line) in schematic.iter().enumerate() {
        let line_len = line.len();
        for (j, c) in line.iter().enumerate() {
            if *c == '*' {
                let mut nums = vec![];
                let mut indices_checked = std::collections::HashSet::new();
                if i > 0 {
                    if j > 0 {
                        match check_index(i - 1, j - 1, &schematic, &indices_checked) {
                            Some((n, indices)) => {
                                nums.push(n);
                                indices_checked.extend(indices);
                            }
                            None => (),
                        }
                    }
                    match check_index(i - 1, j, &schematic, &indices_checked) {
                        Some((n, indices)) => {
                            nums.push(n);
                            indices_checked.extend(indices);
                        }
                        None => (),
                    }
                    if j < line_len - 1 {
                        match check_index(i - 1, j + 1, &schematic, &indices_checked) {
                            Some((n, indices)) => {
                                nums.push(n);
                                indices_checked.extend(indices);
                            }
                            None => (),
                        }
                    }
                }

                if j > 0 {
                    match check_index(i, j - 1, &schematic, &indices_checked) {
                        Some((n, indices)) => {
                            nums.push(n);
                            indices_checked.extend(indices);
                        }
                        None => (),
                    }
                }
                if j < line_len - 1 {
                    match check_index(i, j + 1, &schematic, &indices_checked) {
                        Some((n, indices)) => {
                            nums.push(n);
                            indices_checked.extend(indices);
                        }
                        None => (),
                    }
                }

                if i < num_lines - 1 {
                    if j > 0 {
                        match check_index(i + 1, j - 1, &schematic, &indices_checked) {
                            Some((n, indices)) => {
                                nums.push(n);
                                indices_checked.extend(indices);
                            }
                            None => (),
                        }
                    }
                    match check_index(i + 1, j, &schematic, &indices_checked) {
                        Some((n, indices)) => {
                            nums.push(n);
                            indices_checked.extend(indices);
                        }
                        None => (),
                    }
                    if j < line_len - 1 {
                        match check_index(i + 1, j + 1, &schematic, &indices_checked) {
                            Some((n, indices)) => {
                                nums.push(n);
                                indices_checked.extend(indices);
                            }
                            None => (),
                        }
                    }
                }

                if nums.len() == 2 {
                    r += nums[0] * nums[1];
                }
            }
        }
    }

    Some(r)
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

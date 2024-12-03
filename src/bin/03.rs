use regex::{Regex, RegexSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (i, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let s = a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
        sum += s;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let modified = "do()".to_owned() + input.trim() + "don't()";
    println!("{modified}");
    // return None;
    let to_check = Regex::new(r"(?:do\(\))(.*?)(?:don't\(\))").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [line]) in to_check.captures_iter(&modified).map(|c| c.extract()) {
        // println!("{line}");
        for (m, [a, b]) in re.captures_iter(line).map(|c| c.extract()) {
            // println!("{m}")
            let s = a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
            sum += s;
        }
    }

    Some(sum)
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

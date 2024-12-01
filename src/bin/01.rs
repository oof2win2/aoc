use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second) = input.trim().split("\n").fold((vec![], vec![]), |mut acc, line| {
        let mut parts = line.trim().split("   ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        
        acc.0.push(first.parse::<u32>().unwrap());
        acc.1.push(second.parse::<u32>().unwrap());
        acc
    });

    first.sort();
    second.sort();

    let mut total_dist = 0;
    for i in 0..first.len() {
        let a = first[i];
        let b = second[i];

        let dist = a.abs_diff(b);
        total_dist += dist;
    }

    Some(total_dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut first, mut second) = input.trim().split("\n").fold((vec![], vec![]), |mut acc, line| {
        let mut parts = line.trim().split("   ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        
        acc.0.push(first.parse::<u32>().unwrap());
        acc.1.push(second.parse::<u32>().unwrap());
        acc
    });
    
    let secondmap = second.iter().fold(HashMap::new(), |mut acc, item| {
        let value = acc.get(item).unwrap_or(&0);
        acc.insert(item, value.to_owned() + 1);
        acc
    });

    let similarity = first.iter().fold(0, |mut acc, value| {
        let count = secondmap.get(value).unwrap_or(&0);
        let similarity = value * count;
        acc += similarity;
        acc
    });

Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

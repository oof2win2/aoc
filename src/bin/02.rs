advent_of_code::solution!(2);

fn is_valid(numbers: &Vec<u32>) -> bool {
    let is_always_increasing = numbers.windows(2).fold( true, |acc, items| {
        let prev = items[0];
        let curr = items[1];
        if curr > prev {
            true && acc
        } else {
            false
        }
    });
    let is_always_decreasing = numbers.windows(2).fold( true, |acc, items| {
        let prev = items[0];
        let curr = items[1];
        if curr < prev {
            true && acc
        } else {
            false
        }
    });
    if !is_always_decreasing && !is_always_increasing {
        return false
    }
    let diff_is_ok = numbers.windows(2).fold(true, |isok, items| {
        let prev = items[0];
        let curr = items[1];
        if is_always_increasing && curr - prev < 4 {
            return true && isok
        }
        if is_always_decreasing && prev - curr < 4 {
            return true && isok
        }
        return false
    });
    if diff_is_ok {
        true
    } else {
        false
    }
}

fn parse_data(data: &str) -> Vec<Vec<u32>> {
    data.lines().map(|l| l.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let safecount = input.trim().split("\n").fold(0, |safecount, line| {
        let numbers = line.trim().split(" ").map(str::parse::<u32>).map(Result::unwrap).collect::<Vec<u32>>();
        if is_valid(&numbers) {
            safecount + 1
        } else {
            safecount
        }
    });
    Some(safecount)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let reports = parse_data(input);
    // let mut part1 = 0;
    // let mut part2 = 0;
    // for report in reports {
    //     if is_valid(&report) {
    //         part1 +=1;
    //         part2 +=1;
    //     } else {
    //         for i in 0..report.len(){
    //             let mut candidate = report.clone();
    //             candidate.remove(i);
    //             if is_valid(&candidate)
    //             {
    //                 part2+=1;
    //                 break;
    //             }
    //         }

    //     }
    // }
    // println!("{part2}");


    let safecount = input.trim().split("\n").fold(0, |safecount, line| {
        let numbers = line.trim().split_ascii_whitespace().map(str::parse::<u32>).map(Result::unwrap).collect::<Vec<u32>>();

        let length = numbers.len();

        if is_valid(&numbers) {
            // println!("SAFE");
            return safecount + 1
        }

        for to_remove in 0..length {
            let filtered = &numbers.iter().enumerate().filter(|(i, _)| i != &to_remove).map(|(_, e)| e.to_owned()).collect::<Vec<u32>>();
            // println!("{} {:?} {:?}", to_remove, filtered, numbers);
            if is_valid(&filtered) {
                // println!("SAFE when removing at index {}", to_remove + 1);
                return safecount + 1
            }
        }

        println!("NOT SAFE");
        return safecount
    });

    Some(safecount)
    // None
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

use std::str::Chars;
advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let a: Vec<_> = input
        .lines()
        .map(|line| {
            let mut l: Vec<_> = line.split(' ').collect();
            // let c: Vec<_> = l
            //     .first()
            //     .unwrap()
            //     .split('.')
            //     .filter(|s| !s.is_empty())
            //     .map(|s| s.chars().collect::<Vec<_>>())
            //     .collect();
            let c: Vec<char> = l.first().unwrap().chars().collect();
            let n = l
                .last()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (c, n)
        })
        .collect();

    let ans = find1(&a[0].0, &a[0].1);

    println!("{:?}", a);
    None
}

fn find1(chars: &[char], nums: &[u32]) -> u32 {
    if !chars.contains(&'?') {
        return 0;
    }
    println!("{:?} {:?}", chars, nums);
    0
}

fn valid(chars: &[char], nums: &[u32]) -> bool {
    let mut n = nums.iter();
    let mut curr = n.next().unwrap();
    chars.iter().
    })
}

// fn find_arrangements(mut chars: Vec<Vec<char>>, nums: &[u32]) -> u32 {
//     println!("{:?} {:?}", chars, nums);
//
//     for cc in chars.iter_mut() {
//         for (i, c) in cc.iter().enumerate() {
//             if c == &'?' {
//                 cc[i] = '#';
//             }
//         }
//     }
//     0
// }

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}

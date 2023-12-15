advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let a = input.trim().split(',').map(|s| hash(s) as u32).sum();

    Some(a)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![Vec::new(); 256];
    input.trim().split(',').for_each(|s| {
        if s.ends_with('-') {
            let mut a = s.split('-');
            let key = a.next().unwrap();
            let h = hash(key) as usize;
            let i = get_index(key, &boxes[h]);
            if i.is_some() {
                boxes[h].remove(i.unwrap());
            }
        } else {
            let mut a = s.split('=');
            let key = a.next().unwrap();
            let value = a.next().unwrap().parse::<u8>().unwrap();
            let h = hash(key) as usize;
            match get_index(key, &boxes[h]) {
                Some(i) => boxes[h][i].1 = value,
                None => boxes[h].push((key, value)),
            }
        }
    });

    let a = boxes
        .iter()
        .map(|b| {
            b.iter()
                .enumerate()
                .map(|(i, (k, v))| (hash(k) as u32 + 1) * *v as u32 * (i as u32 + 1))
                .sum::<u32>()
        })
        .sum::<u32>();

    Some(a)
}

fn get_index(needle: &str, haystack: &[(&str, u8)]) -> Option<usize> {
    haystack.iter().position(|&x| x.0 == needle)
}

fn hash(s: &str) -> u8 {
    s.chars()
        .map(|c| c as u32)
        .fold(0, |mut acc, c| {
            acc += c;
            acc *= 17;
            acc %= 256;
            acc
        })
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}

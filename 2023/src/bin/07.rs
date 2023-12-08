use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut all = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|v| (v[0].chars(), v[1].parse::<u32>().unwrap()))
        .map(|(cc, n)| {
            let cc: Vec<u32> = cc.map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).unwrap(),
            }).collect();

            let mut m: HashMap<u32, u32> = HashMap::new();
            for c in cc.clone().into_iter() {
                *m.entry(c).or_insert(0) += 1;
            }
            hand_strength(m.clone());
            (cc, n, m)
        })
        .collect::<Vec<_>>()
        ;

    all.sort_unstable_by(|a, b| {
        let (a_cc, _, a_m) = a;
        let (b_cc, _, b_m) = b;

        let a = hand_strength(a_m.clone());
        let b = hand_strength(b_m.clone());

        if a == b {
            for i in 0..a_cc.len() {
                if a_cc[i] != b_cc[i] {
                    return a_cc[i].cmp(&b_cc[i]);
                }
            }
        }
        a.cmp(&b)
    });

    let mut sum = 0;

    for i in 0..all.len() {
        sum += (i + 1) as u32 * &all[i].1;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut all = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|v| (v[0].chars(), v[1].parse::<u32>().unwrap()))
        .map(|(cc, n)| {
            let cc: Vec<u32> = cc.map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 10,
                'J' => 1,
                _ => c.to_digit(10).unwrap(),
            }).collect();

            let mut m: HashMap<u32, u32> = HashMap::new();
            for c in cc.clone().into_iter() {
                *m.entry(c).or_insert(0) += 1;
            }
            hand_strength(m.clone());
            (cc, n, m)
        })
        .collect::<Vec<_>>()
        ;

    all.sort_unstable_by(|a, b| {
        let (a_cc, _, a_m) = a;
        let (b_cc, _, b_m) = b;

        let a = hand_strength2(a_m.clone());
        let b = hand_strength2(b_m.clone());

        if a == b {
            for i in 0..a_cc.len() {
                if a_cc[i] != b_cc[i] {
                    return a_cc[i].cmp(&b_cc[i]);
                }
            }
        }
        a.cmp(&b)
    });

    let mut sum = 0;

    for i in 0..all.len() {
        sum += (i + 1) as u32 * &all[i].1;
    }

    Some(sum)
}

fn hand_strength(hand: HashMap<u32, u32>) -> u32 {
    let mut cards: Vec<_> = hand.into_iter().collect();
    cards.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let kind = match (cards[0].1, cards.get(1).map(|x| x.1)) {
        (5, _) => 7,
        (4, _) => 6,
        (3, Some(2)) => 5,
        (3, _) => 4,
        (2, Some(2)) => 3,
        (2, _) => 2,
        _ => 0,
    };
    kind
}

fn hand_strength2(hand: HashMap<u32, u32>) -> u32 {
    let mut cards: Vec<_> = hand.into_iter().collect();
    cards.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    // loop and pop 1s out into a new vec
    let mut ones = vec![];
    let mut new_cards = vec![];
    for i in 0..cards.len() {
        if cards[i].0 == 1 {
            ones.push(cards[i]);
        } else {
            new_cards.push(cards[i]);
        }
    }

    cards = new_cards;

    if !ones.is_empty() {
        if ones[0].1 == 5 { // 5 jokers
            return 7;
        }
        cards[0].1 += ones[0].1;
    }

    let kind = match (cards[0].1, cards.get(1).map(|x| x.1)) {
        (5, _) => 7,
        (4, _) => 6,
        (3, Some(2)) => 5,
        (3, _) => 4,
        (2, Some(2)) => 3,
        (2, _) => 2,
        _ => 0,
    };
    kind
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}

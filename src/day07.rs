use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run(contents: String) {
    let total: usize = solve(&contents, get_hand_type, CARD_ORDER);
    println!("total is {total}");

    let total_2: usize = solve(&contents, get_hand_type_p2, CARD_ORDER_2);
    println!("total part 2 is {total_2}");
}

fn solve<F>(contents: &String, hand_type: F, order: [char; 13]) -> usize
where
    F: Fn(&str) -> HandType,
{
    contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            let hand = parts[0];
            let bid: usize = parts[1].parse().unwrap();
            let tpe = hand_type(hand);
            (hand, bid, tpe)
        })
        .sorted_by(|l, r| compare_hands(l, r, order))
        .enumerate()
        .map(|(rank, (_cards, bid, _tpe))| {
            // println!("Hand {cards} is rank {rank} tpe={tpe:?} bid={bid}");
            (rank + 1) * bid
        })
        .sum()
}

fn compare_hands(
    left: &(&str, usize, HandType),
    right: &(&str, usize, HandType),
    order: [char; 13],
) -> Ordering {
    let compare_type = left.2.cmp(&right.2).reverse();
    if compare_type == Ordering::Equal {
        compare_cards(left.0, right.0, order)
    } else {
        compare_type
    }
}

const CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARD_ORDER_2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
fn compare_cards(left: &str, right: &str, order: [char; 13]) -> Ordering {
    left.chars()
        .zip(right.chars())
        .find_map(|(l, r)| {
            let il = order.iter().position(|&c| c == l).unwrap();
            let ir = order.iter().position(|&c| c == r).unwrap();
            if il == ir {
                None
            } else {
                Some(il.cmp(&ir))
            }
        })
        .unwrap_or(Ordering::Equal)
}

fn get_hand_type(hand: &str) -> HandType {
    let hand_set: HashMap<char, usize> = hand
        .chars()
        .into_group_map_by(|&c| c)
        .into_iter()
        .map(|(c, v)| (c, v.len()))
        .collect();
    if hand_set.len() == 1 {
        HandType::Five
    } else if hand_set.len() == 5 {
        HandType::High
    } else if hand_set.len() == 4 {
        HandType::Pair
    } else if hand_set.len() == 3 {
        // two pair or 3 of a kind
        let pairs = hand_set.iter().filter(|(_, &c)| c == 2).count();
        if pairs == 2 {
            HandType::TwoPair
        } else {
            HandType::Three
        }
    } else {
        // FullHouse or Four
        let is_four = hand_set.iter().any(|(_, &c)| c == 4);
        if is_four {
            HandType::Four
        } else {
            HandType::FullHouse
        }
    }
}

fn get_hand_type_p2(hand: &str) -> HandType {
    CARD_ORDER_2
        .iter()
        .skip(1)
        .map(|&c| hand.replace('J', c.to_string().as_str()))
        .map(|h| get_hand_type(h.as_str()))
        .min()
        .unwrap()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    High,
}

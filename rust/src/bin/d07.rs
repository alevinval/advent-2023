use std::{cmp::Ordering, collections::HashMap, iter::zip};

static EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

fn norm(hand: &str) -> String {
    let mut chars: Vec<_> = hand.chars().into_iter().collect();
    chars.sort();
    chars.into_iter().collect()
}

fn score_hand(raw_hand: &str, is_p2: bool) -> u64 {
    let hand = norm(raw_hand);
    let mut prev = 'x';
    let mut value = 0;
    let mut curr_len = 0;
    let mut max_len = 0;
    let mut joker = 0;
    for c in hand.chars() {
        if is_p2 && c == 'J' {
            joker += 1;
            continue;
        }
        if c == prev {
            curr_len += 1;
            continue;
        } else if prev == 'x' {
            prev = c;
            curr_len = 1;
            continue;
        }

        max_len = max_len.max(curr_len);
        value += curr_len * curr_len;
        curr_len = 1;
        prev = c;
    }

    max_len = max_len.max(curr_len);
    value += curr_len * curr_len;

    if is_p2 && joker > 0 {
        value -= max_len * max_len;
        value += (max_len + joker) * (max_len + joker);
    }
    value
}

fn run(input: &str, is_p2: bool) {
    let lines: Vec<_> = input
        .lines()
        .map(|l| {
            let s: Vec<_> = l.split(" ").collect();
            (s[0], u64::from_str_radix(s[1], 10).expect("not a bid"))
        })
        .collect();

    let mut scoremap: HashMap<char, char> = HashMap::new();
    scoremap.insert('A', 'E');
    scoremap.insert('K', 'D');
    scoremap.insert('Q', 'C');

    if is_p2 {
        scoremap.insert('J', '0');
    } else {
        scoremap.insert('J', 'B');
    }

    scoremap.insert('T', 'A');

    let mut entries: Vec<_> = lines
        .iter()
        .map(|(hand, bid)| (hand, bid, score_hand(hand, is_p2)))
        .collect();

    entries.sort_by(|(a_hand, _, a_score), (b_hand, _, b_score)| {
        let d = a_score.cmp(b_score);
        match d {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (a, b) in zip(a_hand.chars(), b_hand.chars()) {
                    if a != b {
                        let ma = scoremap.get(&a).unwrap_or(&a);
                        let mb = scoremap.get(&b).unwrap_or(&b);
                        return ma.cmp(&mb);
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    });

    let ans: u64 = entries
        .iter()
        .enumerate()
        .map(|(rank, (_, bid, _))| (rank + 1) as u64 * (*bid))
        .sum();
    println!("{ans:?}");
}

fn main() {
    let input = advent::open("07");

    println!("PART ONE");
    run(EXAMPLE, false);
    run(&input, false);

    println!("PART TWO");
    run(EXAMPLE, true);
    run(&input, true);
}

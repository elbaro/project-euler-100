extern crate rayon;
extern crate reqwest;

use rayon::prelude::*;

struct Card {
    num: u8,
    value: u8,
    shape: u8,
}

impl Card {
    fn new(s: &str) -> Card {
        let s = s.as_bytes();
        let shape = "HDCS".as_bytes().iter().position(|&c| c == s[1]).unwrap() as u8;
        let num = match s[0] as char {
            'A' => 1,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            _ => s[0] as u8 - '0' as u8,
        };

        let value = if num == 1 { 14 } else { num };
        Card {
            num: num,
            value: value,
            shape: shape,
        }
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(cards: &[&str]) -> Hand {
        Hand {
            cards: cards.iter().map(|s| Card::new(s)).collect(),
        }
    }
    fn value(&self) -> (u32, u32, u32) {
        let mut values: Vec<_> = self.cards.iter().map(|c| c.value).collect();
        values.sort_unstable();

        let mut num = [0; 15];
        for card in self.cards.iter() {
            num[card.num as usize] += 1;
        }
        num.sort_unstable_by(|a, b| b.cmp(a));

        let is_flush = {
            let s = self.cards[0].shape;
            self.cards.iter().all(|c| c.shape == s)
        };
        let is_straight = values[0] + 1 == values[1]
            && values[1] + 1 == values[2]
            && values[2] + 1 == values[3]
            && values[3] + 1 == values[4];

        let highest = (((values[4] as u32 * 15 + values[3] as u32) * 15 + values[2] as u32) * 15
            + values[1] as u32)
            * 15
            + values[0] as u32;

        // royal
        if is_straight && is_flush && values[4] == 13 {
            return (10, 0, highest);
        }

        // straight flush
        if is_straight && is_flush {
            return (9, 0, highest);
        }

        // four cards
        if num[0] == 4 {
            let rank_highest = if values[0] == values[1] {
                values[0]
            } else {
                values[4]
            };
            return (8, rank_highest as u32, highest);
        }

        // full house
        if num[0] == 3 && num[1] == 2 {
            let rank_highest = if values[0] == values[2] {
                values[0]
            } else {
                values[4]
            };
            return (7, rank_highest as u32, highest);
        }

        // flush
        if is_flush {
            return (6, 0, highest);
        }

        // straight
        if is_straight {
            return (5, 0, highest);
        }
        // three cards of a kind
        if num[0] == 3 {
            // 1 1 1 2 3
            // 1 2 2 2 3
            // 1 2 3 3 3
            return (4, values[2] as u32, highest);
        }

        // two pairs
        if num[0] == 2 && num[1] == 2 {
            let (p1, p2) =
            // 1 1 2 2 3
                if values[3]!=values[4] { (values[0], values[2]) }
            // 1 2 2 3 3
                else if values[0]!=values[1] { (values[1], values[3]) }
            // 1 1 2 3 3
                else { (values[0], values[4]) };

            return (3, p1.max(p2) as u32 * 15 + p1.min(p2) as u32, highest);
        }
        // one pair
        if num[0] == 2 {
            let mut rank_highest = 0;
            for i in 0..4 {
                if values[i] == values[i + 1] {
                    rank_highest = values[i];
                    break;
                }
            }
            return (2, rank_highest as u32, highest);
        }
        // highest
        return (1, 0, highest);
    }
}

fn main() {
    let res = reqwest::get("https://projecteuler.net/project/resources/p054_poker.txt")
        .unwrap()
        .text()
        .unwrap();
    let ans = res
        .par_lines()
        .filter(|line| {
            let hands: Vec<&str> = line.split(' ').collect();
            Hand::new(&hands[..5]).value() > Hand::new(&hands[5..]).value()
        })
        .count();
    println!("{}", ans);
}

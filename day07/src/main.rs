const INPUT: &'static str = include_str!("./input");

use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Num(usize),
    Joker,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Card::Ace, Card::Ace) => Ordering::Equal,
            (Card::Ace, _) => Ordering::Greater,
            (_, Card::Ace) => Ordering::Less,

            (Card::King, Card::King) => Ordering::Equal,
            (Card::King, _) => Ordering::Greater,
            (_, Card::King) => Ordering::Less,

            (Card::Queen, Card::Queen) => Ordering::Equal,
            (Card::Queen, _) => Ordering::Greater,
            (_, Card::Queen) => Ordering::Less,

            (Card::Jack, Card::Jack) => Ordering::Equal,
            (Card::Jack, _) => Ordering::Greater,
            (_, Card::Jack) => Ordering::Less,

            (Card::Num(a), Card::Num(b)) => a.cmp(b),
            (Card::Num(_), _) => Ordering::Greater,
            (_, Card::Num(_)) => Ordering::Less,

            (Card::Joker, Card::Joker) => Ordering::Equal
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        if let Some(n) = value.to_digit(10) {
            return Card::Num(n as usize);
        }

        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Num(10),
            _ => panic!()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => Ordering::Equal,
            (HandType::FiveOfAKind, _) => Ordering::Greater,
            (_, HandType::FiveOfAKind) => Ordering::Less,

            (HandType::FourOfAKind, HandType::FourOfAKind) => Ordering::Equal,
            (HandType::FourOfAKind, _) => Ordering::Greater,
            (_, HandType::FourOfAKind) => Ordering::Less,

            (HandType::FullHouse, HandType::FullHouse) => Ordering::Equal,
            (HandType::FullHouse, _) => Ordering::Greater,
            (_, HandType::FullHouse) => Ordering::Less,

            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => Ordering::Equal,
            (HandType::ThreeOfAKind, _) => Ordering::Greater,
            (_, HandType::ThreeOfAKind) => Ordering::Less,

            (HandType::TwoPair, HandType::TwoPair) => Ordering::Equal,
            (HandType::TwoPair, _) => Ordering::Greater,
            (_, HandType::TwoPair) => Ordering::Less,

            (HandType::OnePair, HandType::OnePair) => Ordering::Equal,
            (HandType::OnePair, _) => Ordering::Greater,
            (_, HandType::OnePair) => Ordering::Less,
            
            (HandType::HighCard, HandType::HighCard) => Ordering::Equal,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HandType {
    fn evaluate(cards: [Card; 5]) -> HandType {
        let mut card_counts: HashMap<Card, usize> = HashMap::new();

        for card in cards {
            if let Some(count) = card_counts.get_mut(&card) {
                *count += 1;
            } else {
                card_counts.insert(card, 1);
            }
        }

        match card_counts.len() {
            5 => {
                if let Some(&_) = card_counts.get(&Card::Joker) {
                    return Self::OnePair;
                }

                return Self::HighCard;
            },
            4 => {
                if let Some(&_) = card_counts.get(&Card::Joker) {
                    return Self::ThreeOfAKind;
                }
                
                return Self::OnePair;
            },
            3 => {
                let mut it = card_counts.iter();

                if let Some(&joker_count) = card_counts.get(&Card::Joker) {
                    if joker_count == 3 {
                        return Self::FourOfAKind;
                    }
                    if joker_count == 2 {
                        return Self::FourOfAKind;
                    }
                }

                while let Some((_, &count)) = it.next() {
                    if count == 2 {
                        if let Some(&_) = card_counts.get(&Card::Joker) {
                            return Self::FullHouse;
                        }

                        return Self::TwoPair;
                    }

                    if count == 3 {
                        if let Some(&_) = card_counts.get(&Card::Joker) {
                            return Self::FourOfAKind;
                        }

                        return Self::ThreeOfAKind;
                    }
                }

                panic!()
            }
            2 => { 
                if let Some(&_) = card_counts.get(&Card::Joker) {
                    return Self::FiveOfAKind;
                }

                let &first_count = card_counts.iter().next().unwrap().1;
                if first_count == 2 || first_count == 3 {
                    return Self::FullHouse;
                }  else {
                    return Self::FourOfAKind;
                }
            },
            1 => return Self::FiveOfAKind,
            _ => panic!()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    ty: HandType,
    bid: usize
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ordering = self.ty.cmp(&other.ty);

        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        for c in 0..5 {
            let card_ordering = self.cards[c].cmp(&other.cards[c]);

            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn from(value: &str, use_jokers: bool) -> Self {
        let (cards, bid) = value.split_once(" ").unwrap();
        let cards: Vec<_> = cards.chars().take(5).map(|c|c.into()).collect();
        let mut cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];
        if use_jokers {
            for c in 0..5 {
                if cards[c] == Card::Jack {
                    cards[c] = Card::Joker
                }
            }
        }
        let bid = usize::from_str_radix(bid, 10).unwrap();
        let ty = HandType::evaluate(cards);

        Hand { cards, ty, bid }
    }
}

fn part1() {
    let mut hands: Vec<Hand> = INPUT.lines().map(|l|Hand::from(l, false)).collect();

    hands.sort();

    let winnings: usize = hands.iter().enumerate().map(|(rank, hand)| (rank + 1) * hand.bid).sum();

    println!("Part 1: {}", winnings);
}

fn part2() {
    let mut hands: Vec<Hand> = INPUT.lines().map(|l|Hand::from(l, true)).collect();

    hands.sort();

    let winnings: usize = hands.iter().enumerate().map(|(rank, hand)| (rank + 1) * hand.bid).sum();

    println!("Part 2: {}", winnings);
}

fn main() {
    part1();
    part2();
}

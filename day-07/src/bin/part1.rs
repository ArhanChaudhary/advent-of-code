use std::iter::zip;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(PartialOrd, PartialEq, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    bet: usize,
    cards: [Card; 5],
    hand_type: HandType,
}

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
enum Card {
    FaceCard(FaceCardType),
    NumberCard(usize),
}

impl Card {
    fn from(c: char) -> Card {
        match c {
            'A' => Card::FaceCard(FaceCardType::Ace),
            'K' => Card::FaceCard(FaceCardType::King),
            'Q' => Card::FaceCard(FaceCardType::Queen),
            'J' => Card::FaceCard(FaceCardType::Jack),
            'T' => Card::FaceCard(FaceCardType::Ten),
            _ => Card::NumberCard(c.to_digit(10).unwrap().try_into().unwrap()),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        match (self, other) {
            (Card::FaceCard(face1), Card::FaceCard(face2)) => face1.partial_cmp(face2),
            (Card::NumberCard(num1), Card::NumberCard(num2)) => num1.partial_cmp(num2),
            (Card::FaceCard(_), Card::NumberCard(_)) => Some(Ordering::Greater),
            (Card::NumberCard(_), Card::FaceCard(_)) => Some(Ordering::Less),
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, Eq, Hash)]
enum FaceCardType {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let mut card_counts: HashMap<Card, usize> = HashMap::new();
            let cards: [Card; 5] = split
                .next()
                .unwrap()
                .chars()
                .map(Card::from)
                .fold(Vec::with_capacity(5), |mut collection, card| {
                    if let Some(count) = card_counts.get(&card) {
                        card_counts.insert(card, count + 1);
                    } else {
                        card_counts.insert(card, 1);
                    }
                    collection.push(card);
                    collection
                })
                .try_into()
                .unwrap();
            let bet = split.next().unwrap().parse::<usize>().unwrap();
            let mut card_counts: Vec<usize> = card_counts.into_values().collect();
            card_counts.sort();
            let mut sorted_card_counts = card_counts.into_iter();
            let hand_type = match sorted_card_counts.next().unwrap() {
                5 => HandType::FiveOfAKind,
                2 => HandType::FullHouse,
                1 => match sorted_card_counts.next().unwrap() {
                    4 => HandType::FourOfAKind,
                    2 => HandType::TwoPair,
                    1 => match sorted_card_counts.next().unwrap() {
                        3 => HandType::ThreeOfAKind,
                        1 => match sorted_card_counts.next().unwrap() {
                            2 => HandType::OnePair,
                            1 => HandType::HighCard,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            Hand {
                bet,
                cards,
                hand_type,
            }
        })
        .collect();
    hands.sort_by(
        |hand1, hand2| match hand1.hand_type.partial_cmp(&hand2.hand_type) {
            Some(Ordering::Equal) => zip(hand1.cards, hand2.cards)
                .find_map(|(card1, card2)| match card1.partial_cmp(&card2) {
                    Some(Ordering::Equal) => None,
                    Some(ord) => Some(ord),
                    None => unreachable!(),
                })
                .unwrap(),
            Some(ord) => ord,
            None => unreachable!(),
        },
    );
    hands
        .iter()
        .enumerate()
        .fold(0, |sum, (i, hand)| sum + (i + 1) * hand.bet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 6440);
    }
}

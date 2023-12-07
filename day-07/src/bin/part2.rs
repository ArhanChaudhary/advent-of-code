use std::iter::zip;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
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
            'J' => Card::FaceCard(FaceCardType::Joker),
            'T' => Card::FaceCard(FaceCardType::Ten),
            _ => Card::NumberCard(c.to_digit(10).unwrap().try_into().unwrap()),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        match (self, other) {
            (Card::FaceCard(face1), Card::FaceCard(face2)) => match (face1, face2) {
                (JOKER, JOKER) => Some(Ordering::Equal),
                (JOKER, _) => Some(Ordering::Less),
                (_, JOKER) => Some(Ordering::Greater),
                (_, _) => face1.partial_cmp(face2),
            },
            (Card::NumberCard(num1), Card::NumberCard(num2)) => num1.partial_cmp(num2),
            (Card::FaceCard(face1), Card::NumberCard(_)) => {
                if face1 == JOKER {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Card::NumberCard(_), Card::FaceCard(face2)) => {
                if face2 == JOKER {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, Eq, Hash)]
enum FaceCardType {
    Ace = 14,
    King = 13,
    Queen = 12,
    Joker = 11,
    Ten = 10,
}

const JOKER: &FaceCardType = &FaceCardType::Joker;
const JOKER_CARD: &Card = &Card::FaceCard(FaceCardType::Joker);
fn part2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let mut card_map: HashMap<Card, usize> = HashMap::new();
            let cards: [Card; 5] = split
                .next()
                .unwrap()
                .chars()
                .map(Card::from)
                .fold(Vec::with_capacity(5), |mut collection, card| {
                    if let Some(count) = card_map.get(&card) {
                        card_map.insert(card, count + 1);
                    } else {
                        card_map.insert(card, 1);
                    }
                    collection.push(card);
                    collection
                })
                .try_into()
                .unwrap();
            let bet = split.next().unwrap().parse::<usize>().unwrap();
            let jokers = *card_map.get(JOKER_CARD).unwrap_or(&0);
            card_map.remove(JOKER_CARD);
            let mut card_map: Vec<usize> = card_map.values().cloned().collect();
            card_map.sort();
            if let Some(last_card_count) = card_map.last_mut() {
                *last_card_count += jokers;
            } else {
                card_map.push(5);
            }
            let mut card_map = card_map.iter().cloned();
            let hand_type = match card_map.next().unwrap() {
                5 => HandType::FiveOfAKind,
                2 => HandType::FullHouse,
                1 => match card_map.next().unwrap() {
                    4 => HandType::FourOfAKind,
                    2 => HandType::TwoPair,
                    1 => match card_map.next().unwrap() {
                        3 => HandType::ThreeOfAKind,
                        1 => match card_map.next().unwrap() {
                            2 => HandType::OnePair,
                            1 => HandType::HighCard,
                            _ => panic!(),
                        },
                        _ => panic!(),
                    },
                    _ => panic!(),
                },
                _ => panic!(),
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
                    None => panic!(),
                })
                .unwrap(),
            Some(ord) => ord,
            None => panic!(),
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
    fn part2_test() {
        let result = part2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 5905);
    }
}

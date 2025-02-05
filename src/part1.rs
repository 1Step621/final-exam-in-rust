use std::{
    fmt::Display,
    io::stdin,
    ops::{Deref, DerefMut},
};

use itertools::{iproduct, Itertools};
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl From<u8> for Suit {
    fn from(x: u8) -> Self {
        match x {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            _ => panic!("Cannot convert {} to Suit", x),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Suit::Spade => "♠",
            Suit::Heart => "♥",
            Suit::Diamond => "♦",
            Suit::Club => "♣",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl From<u8> for Rank {
    fn from(x: u8) -> Self {
        match x {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Cannot convert {} to Rank", x),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Clone, Debug)]
struct VecCard(Vec<Card>);

impl Deref for VecCard {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VecCard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Card>> for VecCard {
    fn from(v: Vec<Card>) -> Self {
        Self(v)
    }
}

impl Display for VecCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .enumerate()
                .map(|(i, c)| format!("{}: {}", i, c))
                .join("\n")
        )?;
        Ok(())
    }
}

impl FromIterator<Card> for VecCard {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

fn is_flush(hand: &[Card]) -> bool {
    hand.iter().map(|card| card.suit).all_equal()
}

fn is_twopair(hand: &[Card]) -> bool {
    hand.iter()
        .counts_by(|card| card.rank)
        .values()
        .filter(|&&count| count == 2)
        .count()
        == 2
}

fn is_fullhouse(hand: &[Card]) -> bool {
    hand.iter().unique_by(|card| card.rank).count() == 2
}

pub fn main() {
    let mut rng = rand::rng();
    let stdin = stdin();

    let mut deck = iproduct!(0..4, 1..=13)
        .map(|(s, r)| Card {
            suit: s.into(),
            rank: r.into(),
        })
        .collect::<VecCard>();
    let mut hand;

    deck.shuffle(&mut rng);

    hand = deck.drain(0..5).collect::<VecCard>();
    hand.sort();

    println!("Your hand:");
    println!("{}", hand);

    println!();

    println!("Which cards would you like to discard? (0-4, other to stop)");
    while let Ok(idx) = {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input.trim().parse::<usize>()
    } {
        let Some(card) = hand.get_mut(idx) else { break };
        *card = deck.pop().unwrap();
    }
    hand.sort();

    println!();

    println!("Your new hand:");
    println!("{}", hand);

    println!();

    println!(
        "Your hand is: {}",
        match hand {
            hand if is_flush(&hand) => "Flush!",
            hand if is_fullhouse(&hand) => "Full House!",
            hand if is_twopair(&hand) => "Two Pair!",
            _ => "Nope...",
        }
    );
}

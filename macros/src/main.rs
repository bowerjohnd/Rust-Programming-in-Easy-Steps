use std::fmt::{self, Display, Formatter};
use rand::prelude::*;

#[derive (Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Suit {Clubs, Diamonds, Hearts, Spades}

impl Suit {
    fn random() -> Suit {
        *[Clubs, Diamonds, Hearts, Spades]
            .choose(&mut rand::rng())
            .expect("choose failed")
    }
}

impl Display for Suit {
    fn fmt (&self, f: &mut Formatter<'_>)
        -> Result<(), fmt::Error> {
        write!(f, "{}", match *self {
            Self::Clubs => "clubs",
            Self::Diamonds => "diamonds",
            Self::Hearts => "hearts",
            Self::Spades => "spades"
        })?;
        Ok(())
    }
}

// Compiler error was asking for this.
// Not in book, found in downloaded free resources from ineasysteps.com
use Suit::*;
// ... 

#[derive (Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Card {
    suit: Suit,
    rank: u8
}

impl Card {
    fn new(rank: u8, suit: Suit) -> Card{
        if rank == 0 || rank > 13 { panic!("Invalid rank")}
        Card{rank, suit}
    }
    fn random() -> Card {
        Card::new(rand::rng().random_range(1..=13),
            Suit::random())
    }
}

impl Display for Card {
    fn fmt (&self, f: &mut Formatter<'_>)
        -> Result<(), fmt::Error> {
            let rank = match self.rank {
                1 => "ace",
                11 => "jack",
                12 => "queen",
                13 => "king",
                x => &x.to_string()
            };
            write!(f, "{} of {}", rank, self.suit)?;
            Ok(())
    }
}

fn deal(deck: &mut Vec<Card>) -> Card {
    if deck.is_empty() {
        for suit in [Clubs, Diamonds, Hearts, Spades] {
            for rank in 1..=13 {
                deck.push(Card::new(rank, suit));
            }
        }
        deck.shuffle(&mut rand::rng());
    }
    deck.pop().unwrap()
}

fn print_cards(deck: Vec<Card>, heading: &str) {
    println!("\n{heading}");
    for card in deck {
        println!("  {card}");
    }
}

// Macros

macro_rules! cards {
    ( $num:expr ) => {
        {
            let mut hand = Vec::new();
            for _i in 0..$num {
                hand.push(Card::random());
            }
            hand
        }  
    };

    ( $deck:expr, $num:expr ) => {
        {
            let mut hand = Vec::new();
            for _i in 0..$num {
                hand.push(deal($deck));
            }
            hand
        }
    };

    ( $( ($rank:expr, $suit:ident) ),+ ) => {
        {
            let mut deck = Vec::new();
            $(
                deck.push(Card::new($rank, $suit));
            )+
            deck
        }
    };
}

// Macros go above this comment

fn main() {

    let hand = cards!((2, Spades), (3, Hearts), (11, Clubs));
    print_cards(hand, "Literal Cards");

    let hand = cards!(3);
    print_cards(hand, "Random cards 3x:");

    let mut deck = Vec::new();
    let mut hand = cards!(&mut deck, 5);
    hand.sort();
    print_cards(hand, "Dealt hand");
}


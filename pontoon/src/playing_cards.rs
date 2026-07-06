use rand::seq::SliceRandom;

pub mod pontoon;

#[derive(Copy, Clone)]
pub struct Card {
    suit: u8,
    rank: u8
}

impl Card {
    pub fn to_string(self) -> String {

        let rank = match self.rank {
            1 => "Ace",
            11 => "jack",
            12 => "Queen",
            13 => "King",
            x@2..=10 => &x.to_string()[..],
            x => panic!("Illegal rank: {x}")
        };

        let suit = match self.suit {
            0 => "Clubs",
            1 => "Diamonds",
            2 => "Hearts",
            3 => "Spades",
            x => panic!("Illegal suit: {x}")
        };

        format!("{} of {}", rank, suit)
    }
}

pub fn new_deck() -> Vec<Card> {
    Vec::new()
}

pub fn shuffle(deck: &mut Vec<Card>) {
    deck.clear();

    for suit in 0..4 {
        for rank in 1..=13 {
            deck.push(Card{suit, rank});
        }
    }
    let mut rng = rand::rng();

    deck.shuffle(&mut rng)
}

pub fn deal(deck: &mut Vec<Card>) -> Card {
    if deck.is_empty() {
        shuffle(deck);
    }
    deck.pop().unwrap()
}

pub fn print_cards(cards: &Vec<Card>) {
    for card in cards {
        println!("    {}", card.to_string());
    }
}

pub fn hit(deck: &mut Vec<Card>, hand: &mut Vec<Card>) 
    -> Card {
        let card = deal(deck);
        hand.push(card);
        card
}


use super::Card;

impl Card {
    pub fn pontoon_score(self) -> u8 {
        match self.rank {
            x@1..=10 => x,
            11..=13 => 10,
            x => panic!("Illegal Rank: {x}")
        }
    }
}

pub fn score_hand(hand: &[Card]) -> u8 {
    let mut ace = false;
    let mut total = 0;

    for card in hand {
        let value = card.pontoon_score();
        ace = ace || value == 1;
        total += value;
    }
    if total < 12 && ace {
        total += 10;
    }
    total
}
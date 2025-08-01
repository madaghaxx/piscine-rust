use rand::Rng;
#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen_range(0..4);
        match n {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen_range(0..5);
        match n {
            1 => Rank::Ace,
            11 => Rank::King,
            12 => Rank::Queen,
            13 => Rank::Jack,
            _ => Rank::Number(rng.gen_range(2..=10)),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::King,
            12 => Rank::Queen,
            13 => Rank::Jack,
            _ => Rank::Number(value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    // if let Suit::Club = card.suit {
    //     if let Rank::Ace = card.rank {
    //         return true;
    //     }
    // }
    // false
    Suit::Spade == card.suit && Rank::Ace == card.rank
}

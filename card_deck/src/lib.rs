#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Rank {
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
    King,
    Queen,
    Jack,
}
use rand::Rng;
impl Suit {
    pub fn random() -> Suit {
        let rng = rand::thread_rng().gen_range(1..=4);
        match rng {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => panic!("What the deck"),
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => panic!("What the deck"),
        }
    }
}
impl Rank {
    pub fn random() -> Rank {
        let rng = rand::thread_rng().gen_range(1..=13);
        match rng {
            1 => Self::Ace,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::King,
            12 => Self::Queen,
            13 => Self::Jack,

            _ => panic!("What the deck"),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Self::Ace,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::King,
            12 => Self::Queen,
            13 => Self::Jack,
            _ => panic!("What the deck"),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
  card.suit == Suit::Spade && card.rank == Rank::Ace
}


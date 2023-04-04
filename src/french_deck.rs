use bevy::ecs::component::Component;
use std::fmt;

pub const SUITS: [&str; 4] = ["H", "C", "D", "S"];
pub const RANKS: [&str; 13] = [
    "02", "03", "04", "05", "06", "07", "08", "09", "10", "J", "Q", "K", "A",
];

pub struct CardDeck<'a>(pub Vec<Card<'a>>);

impl CardDeck<'_> {
    pub fn single() -> Self {
        let items = SUITS
            .iter()
            .flat_map(|suit| RANKS.iter().map(move |rank| (suit, rank)))
            .enumerate()
            .map(|(i, tup)| Card {
                suit: tup.0,
                rank: tup.1,
                deck_pos: i,
            })
            .collect();
        Self(items)
    }

    pub fn double() -> Self {
        unimplemented!();
    }
}

#[derive(Component, Clone, Copy)]
pub struct Card<'a> {
    pub suit: &'a str,
    pub rank: &'a str,
    pub deck_pos: usize,
}

impl Card<'_> {
    pub fn file_name(&self) -> String {
        let prefix = "card";
        let suffix = "png";
        let suit_full = match self.suit {
            "H" => "hearts".to_string(),
            "S" => "spades".to_string(),
            "C" => "clubs".to_string(),
            "D" => "diamonds".to_string(),
            _ => " ".to_string(),
        };
        format!("{}_{}_{}.{}", prefix, suit_full, self.rank, suffix)
    }
}

impl fmt::Display for Card<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}_{}", self.deck_pos, self.suit, self.rank)
    }
}

use bevy::ecs::component::Component;
use std::fmt;

pub const SUITS: [&str; 4] = ["H", "C", "D", "S"];
pub const RANKS: [&str; 13] = [
    "02", "03", "04", "05", "06", "07", "08", "09", "10", "J", "Q", "K", "A",
];
pub const DECK_MAX: usize = 4 * 13;

pub struct CardDeck(pub Vec<Card>);

impl CardDeck {
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
        let mut a = Self::single();
        let b = Self::single();
        // iterate b and increase .deck_pos
        let mut b_itms: Vec<Card> =
            b.0.into_iter()
                .map(|mut c| {
                    c.deck_pos = c.deck_pos + DECK_MAX;
                    c
                })
                .collect();
        a.0.append(&mut b_itms);
        Self(a.0)
    }
}

#[derive(Component, Clone)]
pub struct Card {
    pub suit: &'static str,
    pub rank: &'static str,
    pub deck_pos: usize,
}

impl Card {
    pub fn file_name(&self) -> String {
        let prefix = "card";
        let suffix = "png";
        let suit_full = match self.suit {
            "H" => "hearts",
            "S" => "spades",
            "C" => "clubs",
            "D" => "diamonds",
            _ => " ",
        };
        format!("{}_{}_{}.{}", prefix, suit_full, self.rank, suffix)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}_{}", self.deck_pos, self.suit, self.rank)
    }
}

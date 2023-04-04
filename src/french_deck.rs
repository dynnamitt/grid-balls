use bevy::ecs::component::Component;
use std::fmt;

pub const SUITS: [&str; 4] = ["H", "C", "D", "S"];
pub const RANKS: [&str; 13] = [
    "02", "03", "04", "05", "06", "07", "08", "09", "10", "J", "Q", "K", "A",
];

pub struct CardDeck<'a>(pub Vec<Card<'a>>);

impl CardDeck<'_> {
    pub fn new() -> Self {
        let items = SUITS
            .iter()
            .flat_map(|suit| RANKS.iter().map(|rank| Card(suit, rank)))
            .collect();
        Self(items)
    }
}

// who is gonna win?
#[derive(Component)]
pub struct Card<'a>(&'a str, &'a str);

impl Card<'_> {
    pub fn file_name(&self) -> String {
        let prefix = "card";
        let suffix = "png";
        let s = self.0;
        let suit_full = match s {
            "H" => "hearts".to_string(),
            "S" => "spades".to_string(),
            "C" => "clubs".to_string(),
            "D" => "diamonds".to_string(),
            _ => " ".to_string(),
        };
        format!("{}_{}_{}.{}", prefix, suit_full, self.1, suffix)
    }
}

impl fmt::Display for Card<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.0, self.1)
    }
}

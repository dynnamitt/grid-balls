use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        let timer = Timer::from_seconds(1.0, TimerMode::Repeating);

        app.add_startup_system(setup_card_deck)
            .add_startup_system(load_large_cards)
            .insert_resource(GreetTimer(timer))
            .add_system(all_cards_txt_dump);
        // .add_system(greet_people);
    }
}

#[derive(Resource, Reflect, Default)]
struct GreetTimer(Timer);

#[derive(Resource, Default)]
struct CardAssets(Vec<HandleUntyped>);

fn load_large_cards(mut commands: Commands, server: Res<AssetServer>) {
    if let Ok(handles) = server.load_folder("large_cards") {
        commands.insert_resource(CardAssets(handles));
    }
}

// --------------------------------- comps
//
mod french_deck {
    use std::fmt;

    pub const RANK_MAX: usize = 13;
    pub const SUIT_MAX: usize = 4;

    fn rank_str(r: usize) -> String {
        if r > 8 {
            ["J", "Q", "K", "A"][r - 9].to_string()
        } else {
            (r + 2).to_string()
        }
    }

    fn suit_str(s: usize) -> String {
        match s {
            0 => "H".to_string(),
            1 => "S".to_string(),
            2 => "C".to_string(),
            3 => "D".to_string(),
            _ => " ".to_string(),
        }
    }

    pub struct Card(pub usize, pub usize);

    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}{}", suit_str(self.0), rank_str(self.1))
        }
    }
}

#[derive(Component)]
struct Suit(usize);

#[derive(Component)]
struct Rank(usize);

fn setup_card_deck(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for s in 0..french_deck::SUIT_MAX {
        for r in 0..french_deck::RANK_MAX {
            let s_comp = Suit(s);
            let r_comp = Rank(r);
            commands.spawn((s_comp, r_comp));
        }
    }
}

fn all_cards_txt_dump(time: Res<Time>, mut timer: ResMut<GreetTimer>, q: Query<(&Suit, &Rank)>) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut prev_s: usize = 0;
        for (suit, rank) in q.iter() {
            if suit.0 != prev_s {
                println!("");
            }
            prev_s = suit.0;
            print!("{} ", french_deck::Card(suit.0, rank.0));
        }
        println!(".")
    }
}

/*
                 _
 _ __ ___   __ _(_)_ __
| '_ ` _ \ / _` | | '_ \
| | | | | | (_| | | | | |
|_| |_| |_|\__,_|_|_| |_|

*/
fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // PluginGroup
        .add_plugin(HelloPlugin) // single
        .add_plugin(WorldInspectorPlugin::new()) // single
        .run();
}

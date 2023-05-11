use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;

mod french_deck;

pub struct CardGamePlugin;

impl Plugin for CardGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("cecece").unwrap()))
            .add_startup_system(setup_card_deck)
            .add_startup_system(spawn_camera.before(setup_card_deck));
        // .add_system(animate_cards);
    }
}

// --------------------------------- comps
//

fn spawn_camera(mut commands: Commands, win_query: Query<&Window, With<PrimaryWindow>>) {
    let win: &Window = win_query.get_single().unwrap();

    let xc = win.width() / 2.0;
    let yc = win.height() / 2.0;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(xc, yc, 200.0),
        ..default()
    });
}

// const CARD_SPEED: f32 = 250.0;
// const CARD_WIDTH: f32 = 64.0;

// fn animate_cards(
//     time: Res<Time>,
//     // card_query: Query<(&mut Transform, &french_deck::Card)>,
//     win_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//     let win: &Window = win_query.get_single().unwrap();
//     unimplemented!();
// }

#[derive(Component)]
struct StartStackPos(usize);

#[derive(Component)]
struct Stack(usize);

fn setup_card_deck(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    let win: &Window = win_query.get_single().unwrap();

    let startdeck_x = win.width() / 2.0; //- CARD_WIDTH / 2.0;
    let startdeck_y = win.height() / 12.0 * 11.0; // - CARD_WIDTH / 2.0;

    // println!("startdeck = {},{}", startdeck_x, startdeck_y);

    let dir_prefix = "large_cards";

    // You can load all assets in a folder like this.
    // They will be loaded in parallel without blocking
    let _card_assets: Vec<HandleUntyped> = asset_server.load_folder(dir_prefix).unwrap();

    // Then any asset in the folder can be accessed like this:
    // let monkey_handle = asset_server.get_handle("models/monkey/Monkey.png");

    for card in french_deck::CardDeck::double().0 {
        let img_path = format!("{}/{}", dir_prefix, card.file_name());
        println!("{}", card);
        let card_hnd: Handle<Image> = asset_server.get_handle(&img_path);
        let deck_position = card.deck_pos as f32;
        let x = startdeck_x + (deck_position / 3.0);
        let y = startdeck_y + (deck_position / 3.0);
        let z = deck_position;
        commands.spawn((
            Name::new(card.to_string()),
            card,
            SpriteBundle {
                texture: card_hnd,
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
        ));
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
        .add_plugin(CardGamePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DebugLinesPlugin::default())
        .add_system(show_grid)
        .run();
}

fn show_grid(win_query: Query<&Window, With<PrimaryWindow>>, mut lines: ResMut<DebugLines>) {
    let win: &Window = win_query.get_single().unwrap();

    for x in (0..win.width() as usize).step_by(64) {
        let start = Vec3::new(x as f32, 0., 0.);
        let end = Vec3::new(x as f32, win.height(), 0.);
        lines.line_colored(start, end, 0., Color::GREEN);
    }
    for y in (0..win.height() as usize).step_by(64) {
        let start = Vec3::new(0., y as f32, 0.);
        let end = Vec3::new(win.width(), y as f32, 0.);
        lines.line_colored(start, end, 0., Color::GREEN);
    }
    // TODO: strange padding when smaller windo
}

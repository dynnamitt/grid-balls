use bevy::{prelude::*, window::PrimaryWindow};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod french_deck;

pub struct CardGamePlugin;

impl Plugin for CardGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_card_deck)
            .add_startup_system(spawn_camera.before(setup_card_deck));
    }
}

// --------------------------------- comps
//

fn spawn_camera(mut commands: Commands, win_query: Query<&Window, With<PrimaryWindow>>) {
    let win: &Window = win_query.get_single().unwrap();

    let xc = win.width() / 2.0;
    let yc = win.height() / 2.0;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(xc, yc, 0.0),
        ..default()
    });
}

fn setup_card_deck(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    let win: &Window = win_query.get_single().unwrap();

    let x_c = win.width() / 2.0;
    let y_c = win.height() / 2.0;

    let dir_prefix = "large_cards";

    // You can load all assets in a folder like this.
    // They will be loaded in parallel without blocking
    let _card_assets: Vec<HandleUntyped> = asset_server.load_folder(dir_prefix).unwrap();

    // Then any asset in the folder can be accessed like this:
    // let monkey_handle = asset_server.get_handle("models/monkey/Monkey.gltf#Mesh0/Primitive0");

    for c in french_deck::CardDeck::new().0 {
        let img_path = format!("{}/{}", dir_prefix, c.file_name());
        println!("{}", img_path);
        let card_hnd: Handle<Image> = asset_server.get_handle(&img_path);
        commands.spawn((
            c,
            SpriteBundle {
                texture: card_hnd,
                transform: Transform::from_xyz(x_c, y_c, 0.0),
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
        .add_plugin(CardGamePlugin) // single
        // .add_plugin(WorldInspectorPlugin::new()) // single
        .run();
}

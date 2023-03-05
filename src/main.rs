use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        let timer = Timer::from_seconds(1.0, TimerMode::Repeating);
        app.add_startup_system(add_people)
            .insert_resource(GreetTimer(timer))
            .add_system(greet_people);
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct InHuman;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
    commands.spawn((Person, InHuman));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
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
        // .add_plugin(WorldInspectorPlugin) // single
        .run();
}

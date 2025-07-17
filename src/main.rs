use bevy::prelude::*;
use my_bevy_game::plugins::hello_plugin::HelloPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}
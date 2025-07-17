use bevy::prelude::*;
use my_bevy_game::plugins::hello_plugin::HelloPlugin;
use my_bevy_game::plugins::asset_plugin::AssetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_plugins(AssetPlugin)
        .run();
}

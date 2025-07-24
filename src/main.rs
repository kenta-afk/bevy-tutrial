use bevy::prelude::*;
use my_bevy_game::plugins::animation_plugin::AnimationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(AnimationPlugin)
        .run();
}

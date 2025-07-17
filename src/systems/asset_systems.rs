use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Sprite::from_image(
        asset_server.load("gojo.jpeg"),
    ));
}
use bevy::prelude::*;

use crate::systems::asset_systems::setup;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
use bevy::prelude::*;

use crate::systems::animation_systems::{
    change_direction, execute_animations, setup_sprites
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sprites).add_systems(
            Update,
            (execute_animations, change_direction)
        );
    }
}

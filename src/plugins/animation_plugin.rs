use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use crate::components::animation::LeftSprite;
use crate::systems::animation_systems::{execute_animations, setup_sprites, trigger_animation};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sprites)
            .add_systems(Update, execute_animations)
            .add_systems(
                Update,
                (
                    // Press the left arrow key to animate the left sprite
                    trigger_animation::<LeftSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                )
            );
    }
}

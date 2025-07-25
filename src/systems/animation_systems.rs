use bevy::prelude::*;

use crate::components::animation::{AnimationConfig, Character, Direction};

// This system changes the character's direction and animation when arrow keys are pressed
pub fn change_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Character, &mut AnimationConfig, &mut Sprite)>
) {
    for (mut character, mut animation_config, mut sprite) in &mut query {
        if input.just_pressed(KeyCode::ArrowRight) && character.current_direction != Direction::Right {
            character.current_direction = Direction::Right;
            *animation_config = character.move_right_config.clone();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation_config.first_sprite_index;
            }
        } else if input.just_pressed(KeyCode::ArrowLeft) && character.current_direction != Direction::Left {
            character.current_direction = Direction::Left;
            *animation_config = character.move_left_config.clone();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation_config.first_sprite_index;
            }
        } else if input.just_pressed(KeyCode::ArrowUp) && character.current_direction != Direction::Forward {
            character.current_direction = Direction::Forward;
            *animation_config = character.move_forward_config.clone();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation_config.first_sprite_index;
            }
        } else if input.just_pressed(KeyCode::ArrowDown) && character.current_direction != Direction::Backward {
            character.current_direction = Direction::Backward;
            *animation_config = character.move_backward_config.clone();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation_config.first_sprite_index;
            }
        }
    }
}

// This system runs when the user clicks the left arrow key or right arrow key
pub fn trigger_animation(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut AnimationConfig, With<Character>>
) {
    if input.just_pressed(KeyCode::ArrowRight) || input.just_pressed(KeyCode::ArrowLeft) || input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::ArrowDown) {
        for mut animation in &mut query {
            // We create a new timer when the animation is triggered
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
        }
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = config.first_sprite_index;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

pub fn setup_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands.spawn(Camera2d);

    // Create a minimal UI explaining how to interact with the example
    commands.spawn((
        Text::new("Hero design in Pokemon Game"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }
    ));

    // Load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("gabe-idle-run.png");

    // The sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The first (left-hand) sprite runs at 20 FPS
    let move_right_config = AnimationConfig::new(0, 6, 20);
    let move_left_config = AnimationConfig::new(7, 13, 20);
    let move_backward_config = AnimationConfig::new(14, 16, 20);
    let move_forward_config = AnimationConfig::new(21, 25, 20);

    // Create a single character that can move in both directions
    let character = Character {
        move_right_config: move_right_config.clone(),
        move_left_config: move_left_config.clone(),
        move_backward_config: move_backward_config.clone(),
        move_forward_config: move_forward_config.clone(),
        current_direction: Direction::Right,
    };

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: move_right_config.first_sprite_index
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
        move_right_config, // Start with right movement animation
        character,
    ));
}

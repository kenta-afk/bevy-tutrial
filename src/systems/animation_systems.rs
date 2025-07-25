use bevy::prelude::*;

use crate::components::animation::{AnimationConfig, Character, Direction};

// This system changes the character's direction and animation when arrow keys are pressed
pub fn change_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Character, &mut AnimationConfig, &mut Sprite)>
) {
    for (mut character, mut animation_config, mut sprite) in &mut query {
        let mut new_direction = None;
        let mut new_config = None;
        
        // Check if any movement key is pressed
        if input.pressed(KeyCode::ArrowRight) {
            new_direction = Some(Direction::Right);
            new_config = Some(character.move_right_config.clone());
            character.is_moving = true;
        } else if input.pressed(KeyCode::ArrowLeft) {
            new_direction = Some(Direction::Left);
            new_config = Some(character.move_left_config.clone());
            character.is_moving = true;
        } else if input.pressed(KeyCode::ArrowUp) {
            new_direction = Some(Direction::Forward);
            new_config = Some(character.move_forward_config.clone());
            character.is_moving = true;
        } else if input.pressed(KeyCode::ArrowDown) {
            new_direction = Some(Direction::Backward);
            new_config = Some(character.move_backward_config.clone());
            character.is_moving = true;
        } else {
            // No keys pressed - go to idle
            if character.is_moving {
                new_direction = Some(Direction::Idle);
                new_config = Some(character.idle_config.clone());
                character.is_moving = false;
            }
        }
        
        // Update direction and animation config if changed
        if let (Some(direction), Some(config)) = (new_direction, new_config) {
            if character.current_direction != direction {
                character.current_direction = direction;
                *animation_config = config;
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = animation_config.first_sprite_index;
                }
            }
        }
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite, &Character)>) {
    for (mut config, mut sprite, character) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());
        
        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                // If we're moving, loop the animation, otherwise stay on first frame
                if character.is_moving {
                    atlas.index = config.first_sprite_index;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
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
    let idle_config = AnimationConfig::new(0, 0, 1);

    // Create a single character that can move in both directions
    let character = Character {
        move_right_config: move_right_config.clone(),
        move_left_config: move_left_config.clone(),
        move_backward_config: move_backward_config.clone(),
        move_forward_config: move_forward_config.clone(),
        idle_config: idle_config.clone(),
        current_direction: Direction::Idle,
        is_moving: false
    };

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: idle_config.first_sprite_index
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
        idle_config,
        character
    ));
}

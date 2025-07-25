use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps)
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
pub struct Character {
    pub move_right_config: AnimationConfig,
    pub move_left_config: AnimationConfig,
    pub move_backward_config: AnimationConfig,
    pub move_forward_config: AnimationConfig,
    pub idle_config: AnimationConfig,
    pub current_direction: Direction,
    pub is_moving: bool
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Backward,
    Forward,
    Idle
}

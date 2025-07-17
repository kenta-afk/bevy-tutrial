use bevy::prelude::*;

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

impl Default for GreetTimer {
    fn default() -> Self {
        GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

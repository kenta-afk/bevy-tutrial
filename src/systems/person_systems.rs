use bevy::prelude::*;

use crate::components::person::{Name, Person};
use crate::resources::timer::GreetTimer;

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name::new("Alice".to_string())));
    commands.spawn((Person, Name::new("Bob".to_string())));
    commands.spawn((Person, Name::new("Charlie".to_string())));
}

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello, {}!", name.get());
        }
    }
}

pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.get() == "Alice" {
            name.set("Alice Smith".to_string());
        }
    }
}

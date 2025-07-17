use bevy::prelude::*;
use crate::systems::person_systems::{add_people, greet_people, update_people};
use crate::resources::timer::GreetTimer;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer::default());
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

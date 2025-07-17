use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Name(name)
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn set(&mut self, name: String) {
        self.0 = name;
    }
}
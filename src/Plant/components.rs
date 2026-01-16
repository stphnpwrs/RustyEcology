use bevy::prelude::*;

#[derive(Component)]
pub struct Plant {
    pub age: u8,
    pub dead: bool,
}
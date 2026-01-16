use bevy::prelude::*;

#[derive(Component)]
pub struct Predator {
    pub age: u8,
    pub energy: f32,
    pub dead: bool,
    pub strength: f32,
    pub speed: u8,
    pub cycle_move: u8,
    pub idx_x: f32,
    pub idx_y: f32,
}
use bevy::prelude::*;
use rand::prelude::*;

use crate::World::systems::*;

#[derive(Resource)]
pub struct Grid {
    pub cells: Vec<f32>,
}

#[derive(Component)]
pub struct Tile {
    pub index: usize,
}

impl Grid {

    pub fn init() -> Self {
        let mut rng = rand::rng();
        let cells = vec![0;(GRID_HEIGHT*GRID_WIDTH) as usize]
            .iter()
            .map(|_| rng.random::<f32>())
            .collect();
        Self {cells}
    }

}

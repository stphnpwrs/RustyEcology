use bevy::prelude::*;
use bevy::render::view::screenshot::*;

use crate::World::components::*;
use crate::Predator::components::*;
use crate::Prey::components::*;

pub const SAVE_SCREENSHOTS: bool = true;
pub const GRID_WIDTH: u32 = 32;
pub const GRID_HEIGHT: u32 = 32;
pub const GRID_PADDING: f32 = 1.;
pub const CELL_SIZE: f32 = 20.;

pub const WIN_WIDTH: u32 = GRID_WIDTH * (CELL_SIZE + GRID_PADDING) as u32;
pub const WIN_HEIGHT: u32 = GRID_HEIGHT * (CELL_SIZE + GRID_PADDING) as u32;

pub const TER_ROCK: Color = Color::srgb(0.41, 0.41, 0.41);
pub const TER_SAND: Color = Color::srgb(0.902, 0.749, 0.502);
pub const TER_DIRT: Color = Color::srgb(0.49, 0.298, 0.137);
pub const TER_GRASS: Color = Color::srgb(0.298, 0.549, 0.216);

pub fn init_board(mut commands: Commands) {

    commands.spawn(Camera2d);

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
           let idx = reduce_dim(x as usize, y as usize);
            commands.spawn((
                Sprite::from_color(Color::srgb(1., 1., 1.), vec2(CELL_SIZE, CELL_SIZE)),
                Transform::from_xyz(
                    idx_to_pixel(x as f32),
                    idx_to_pixel(y as f32),  
                    0.),
                Tile { index: idx },
            ));
        }
    }
}

pub fn render_grid(grid: Res<Grid>, mut board_query: Query<(&Tile, &mut Sprite)>) { 

    for (tile, mut sprite) in &mut board_query {

        if grid.cells[tile.index] < 0.3 {
            sprite.color = TER_ROCK;
        } else if grid.cells[tile.index] < 0.5 {
            sprite.color = TER_SAND;
        } else if grid.cells[tile.index] < 0.75 {
            sprite.color = TER_DIRT;
        } else {
            sprite.color = TER_GRASS;
        }
    }
}

pub fn idx_to_pixel(x: f32) -> f32 {
    return x * (CELL_SIZE + GRID_PADDING) - (WIN_WIDTH as f32/ 2.) + (CELL_SIZE/2.);
}

pub fn pixel_to_idx(x: f32) -> f32 {
    return (x - CELL_SIZE/2. + WIN_WIDTH as f32/2.) / (CELL_SIZE + GRID_PADDING);
}

pub fn reduce_dim(x: usize, y: usize) -> usize {
    return y * GRID_WIDTH as usize + x;
}

pub fn increase_dim(idx: usize) -> Vec<usize> {
    let y: usize = idx / GRID_WIDTH as usize;
    let x: usize = idx - (y*GRID_WIDTH as usize);
    return vec![x, y];
}

pub fn in_range(t1: &Transform, t2: &Transform, range:f32) -> bool {
    let dist = t1.translation.distance(t2.translation);
    return dist <= range * (GRID_PADDING+CELL_SIZE);
}

pub fn fertilize(_idx_x: f32, _idx_y: f32) {
    // not yet implemented
    // should contribute to the terrain value at idx_x, idx_y
    // possibly also contribute to surrounding tiles to a lesser extent
}

pub fn goto(x1:Vec2, x2:Vec2, speed: u8, aim: bool) -> Vec2 {

    let mut movement = speed as f32 * (CELL_SIZE + GRID_PADDING);

    let delta = x2-x1;
    let full_dist = (delta[0]*delta[0] + delta[1]*delta[1]).sqrt();
    if aim {
        if full_dist < movement  {
            movement = full_dist;
        }
    }

    let angle = delta[1].atan2(delta[0]);
    let x_idx = pixel_to_idx(x1[0] + angle.cos()*movement) as usize;
    let y_idx = pixel_to_idx(x1[1] + angle.sin()*movement) as usize;
    
    return vec2(x_idx as f32, y_idx as f32);
}

pub fn reset_movement(pred_query: Query<&mut Predator>, prey_query: Query<&mut Prey>) {
  
    for mut predator in pred_query {
        predator.cycle_move = predator.speed;
    }

    for mut prey in prey_query {
        prey.cycle_move = prey.speed;
    }
}

pub fn save_screenshot(mut commands: Commands, mut counter: Local<u32>, ) {

    if SAVE_SCREENSHOTS {
        let path = format!("./src/images/screenshot-{}.png", *counter);
        *counter += 1;

        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }

}
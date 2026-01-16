pub mod Predator;
pub mod Prey;
pub mod Plant;
pub mod World;

use bevy::prelude::*;

use World::components::*;
use World::systems::*;
use Predator::systems::*;
use Prey::systems::*;
use Plant::systems::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Predator Prey Ecology".into(),
            resolution: (
                WIN_WIDTH, 
                WIN_HEIGHT
            ).into(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(Grid::init())
    .add_systems(Startup, (
        init_board, 
        spawn_predator, 
        spawn_prey, 
        spawn_plant))
    .add_systems(Update, (
        reset_movement,
        pred_move,
        prey_move,
        hunt,
        consume,
        predator_reproduce,
        prey_reproduce,
        plant_reproduce,
        pred_die,
        prey_die,
        plant_die,
        render_grid, 
        render_predator, 
        render_prey, 
        render_plant,
        save_screenshot))
    .run();
}

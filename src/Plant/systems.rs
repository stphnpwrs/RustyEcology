use bevy::prelude::*;

use crate::World::systems::*;
use crate::World::components::*;
use crate::Plant::components::*;

pub const PLANT_SPAWN: u32 = 15;
pub const PLANT_REPRO_SUCCESS: f32 = 0.005;
pub const PLANT_DIE_SUCCESS: f32 = 0.9;
pub const PLANT_LIFESPAN: f32 = 5.;

pub const COL_PLANT: Color = Color::srgb(1., 0., 0.);
pub const COL_PLANT_D: Color = Color::srgb(0.35, 0., 0.);

pub fn spawn_plant(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for _ in 0..PLANT_SPAWN {

        let x = rand::random::<f32>() * GRID_WIDTH as f32;
        let y = rand::random::<f32>() * GRID_HEIGHT as f32;
        commands.spawn((
            Plant {age: 0, dead: false},
            Mesh2d(meshes.add(Circle::new(10.)).into()),
            MeshMaterial2d(materials.add(COL_PLANT)),
            Transform::from_xyz(
                    idx_to_pixel(x as u8 as f32),
                    idx_to_pixel(y as u8 as f32),   
                    0.1),
        ));
    }
}

pub fn plant_reproduce(grid: Res<Grid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
    ) {

    for idx in 0..(GRID_HEIGHT * GRID_WIDTH) {

        let new_threshold = PLANT_REPRO_SUCCESS * grid.cells[idx as usize];

        if rand::random::<f32>() < new_threshold {

            let tile_pos = increase_dim(idx as usize);

            commands.spawn((
                Plant {age: 0, dead: false},
                Mesh2d(meshes.add(Circle::new(10.)).into()),
                MeshMaterial2d(materials.add(COL_PLANT)),
                Transform::from_xyz(
                            idx_to_pixel(tile_pos[0] as f32),
                            idx_to_pixel(tile_pos[1] as f32),  
                            0.1),
            ));
        }

    }
}

pub fn plant_die(
    mut commands: Commands, 
    mut plant_query: Query<(Entity, &mut Plant)>
    ) {

    for (plant_id, plant) in &mut plant_query.iter_mut().filter(|(_, p)| !p.dead) {
        if rand::random::<f32>() < plant.age as f32 / PLANT_LIFESPAN * PLANT_DIE_SUCCESS {
            commands.entity(plant_id).despawn();
        }
    }
}

pub fn render_plant(mut plant_render_query: Query<(&Plant, &mut MeshMaterial2d<ColorMaterial>)>, mut materials: ResMut<Assets<ColorMaterial>>) { 

    for (plant, mat) in &mut plant_render_query {
        if let Some(material) = materials.get_mut(mat.id()) {
            if plant.dead {
                material.color = COL_PLANT_D;
            } else { 
                material.color = COL_PLANT;
            }
        }
    }
}
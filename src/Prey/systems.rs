use bevy::prelude::*;

use crate::World::systems::*;
use crate::Prey::components::Prey;
use crate::Plant::components::Plant;

pub const PREY_SPAWN: u32 = 25;
pub const PREY_REPRO_SUCCESS: f32 = 0.8;
pub const PREY_DIE_SUCCESS: f32 = 0.5;
pub const PREY_LIFESPAN: f32 = 10.;

pub const COL_PREY: Color = Color::srgb(1., 0., 0.91);
pub const COL_PREY_H: Color = Color::srgb(1., 0.71, 0.973);
pub const COL_PREY_D: Color = Color::srgb(0.231, 0., 0.208);

pub fn spawn_prey(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {

    for _ in 0..PREY_SPAWN {

        let x = rand::random::<f32>() * GRID_WIDTH as f32;
        let y = rand::random::<f32>() * GRID_HEIGHT as f32;
        commands.spawn((
            Prey {age: 0, energy: 0., dead: false, strength: 1., speed: 3, cycle_move: 3, idx_x:x, idx_y:y},
            Mesh2d(meshes.add(Circle::new(10.)).into()),
            MeshMaterial2d(materials.add(COL_PREY_H)),
            Transform::from_xyz(
                    idx_to_pixel(x as u8 as f32),
                    idx_to_pixel(y as u8 as f32),  
                    0.1),
        ));
    }
}

pub fn prey_reproduce (
        mut commands: Commands,
        mut prey_query: Query<&mut Prey>,
        mut meshes: ResMut<Assets<Mesh>>, 
        mut materials: ResMut<Assets<ColorMaterial>>
        ) {

    for mut prey in prey_query.iter_mut().filter(|p| p.energy >= 1. && !p.dead) {
            
        let mut new_x = prey.idx_x + 1.;
        let mut new_y = prey.idx_y;
        if new_x >= GRID_WIDTH as f32 {
            new_x -= 1.;
            new_y += 1.;
        }
        
        if rand::random::<f32>() < PREY_REPRO_SUCCESS {
            commands.spawn((
                Prey {age: 0, 
                    energy: 0., 
                    dead: false, 
                    strength: prey.strength, 
                    speed: prey.speed, 
                    cycle_move: prey.speed,
                    idx_x:new_x, 
                    idx_y:new_y},
                Mesh2d(meshes.add(Circle::new(10.)).into()),
                MeshMaterial2d(materials.add(COL_PREY_H)),
                Transform::from_xyz(
                            idx_to_pixel(new_x),
                            idx_to_pixel(new_y),  
                            0.1),
            ));

            prey.energy = 0.;
        }
    }
}


pub fn prey_die(
    mut prey_query: Query<&mut Prey>) {


    for mut prey in &mut prey_query.iter_mut().filter(|p| !p.dead) {
        if rand::random::<f32>() < prey.age as f32 / PREY_LIFESPAN * PREY_DIE_SUCCESS {
            prey.dead = true;
            fertilize(prey.idx_x, prey.idx_y);
        }
    }
}


pub fn prey_move(mut prey_query: Query<(&mut Prey, &Transform)>, plant_query: Query<(&Plant, &Transform)>) {

    for (mut prey, prey_trans) in prey_query.iter_mut().filter(|(p, _)| p.energy < 1. && !p.dead) {

        let closest_food = plant_query
            .iter()
            .filter(|(p, _)| !p.dead)
            .min_by(|(_, p_tran1), (_, p_tran2)| {
                let dist1 = prey_trans.translation.distance(p_tran1.translation);
                let dist2 = prey_trans.translation.distance(p_tran2.translation);
                dist1.partial_cmp(&dist2).unwrap()
            });


        match closest_food {
            Some(m) => {
                let prey_new_pos = goto(vec2(prey_trans.translation.x, prey_trans.translation.y), vec2(m.1.translation.x, m.1.translation.y), prey.speed, true);
                prey.idx_x = prey_new_pos[0];
                prey.idx_y = prey_new_pos[1];
            }
            None => println!("No food"),
        }

    }
}

pub fn render_prey(mut prey_render_query: Query<(&Prey, &mut MeshMaterial2d<ColorMaterial>, &mut Transform)>, mut materials: ResMut<Assets<ColorMaterial>>) { 

    for (prey, mat,  mut tran) in &mut prey_render_query {
        
        tran.translation.x = idx_to_pixel(prey.idx_x as f32);
        tran.translation.y = idx_to_pixel(prey.idx_y as f32);

        if let Some(material) = materials.get_mut(mat.id()) {
            if prey.dead {
                material.color = COL_PREY_D;
            } else if prey.energy >= 1. {
                material.color = COL_PREY;
            } else { 
                material.color = COL_PREY_H;
            }
        }
    }
}
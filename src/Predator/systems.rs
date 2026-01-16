use bevy::prelude::*;

use crate::World::systems::*;

use crate::Predator::components::*;
use crate::Prey::components::*;
use crate::Plant::components::*;

pub const COL_PRED: Color = Color::srgb(0., 0.78, 1.);
pub const COL_PRED_H: Color = Color::srgb(0.737, 0.941, 0.914);
pub const COL_PRED_D: Color = Color::srgb(0., 0.082, 0.302);

pub const PRED_SPAWN: u32 = 15;
pub const HUNT_SUCCESS: f32 = 0.4;
pub const PRED_REPRO_SUCCESS: f32 = 0.2;
pub const PRED_DIE_SUCCESS: f32 = 0.5;
pub const PRED_LIFESPAN: f32 = 10.;


pub fn spawn_predator(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {

    for _ in 0..PRED_SPAWN {

        let x = rand::random::<f32>() * GRID_WIDTH as f32;
        let y = rand::random::<f32>() * GRID_HEIGHT as f32;
        commands.spawn((
            Predator {age: 0, energy: 0., dead: false, strength: 1., speed: 2, cycle_move: 2, idx_x:x, idx_y:y},
            Mesh2d(meshes.add(Circle::new(10.)).into()),
            MeshMaterial2d(materials.add(COL_PRED_H)),
            Transform::from_xyz(
                        idx_to_pixel(x as u8 as f32),
                        idx_to_pixel(y as u8 as f32),  
                        0.1),
        ));
    }
}

pub fn predator_reproduce(
    mut commands: Commands,
    mut pred_query: Query<&mut Predator>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
    ) {

    for mut pred in &mut pred_query.iter_mut().filter(|p| p.energy >= 1. && !p.dead) {        
        let mut new_x = pred.idx_x + 1.;
        let mut new_y = pred.idx_y;
        if new_x >= GRID_WIDTH as f32 {
            new_x -= 1.;
            new_y += 1.;
        }
        
        if rand::random::<f32>() < PRED_REPRO_SUCCESS {
            commands.spawn((
                Predator {age: 0, 
                    energy: 0., 
                    dead: false, 
                    strength: pred.strength, 
                    speed: pred.speed, 
                    cycle_move: pred.speed,
                    idx_x:new_x, 
                    idx_y:new_y},
                Mesh2d(meshes.add(Circle::new(10.)).into()),
                MeshMaterial2d(materials.add(COL_PRED_H)),
                Transform::from_xyz(
                            idx_to_pixel(new_x),
                            idx_to_pixel(new_y),  
                            0.1),
            ));

            pred.energy = 0.;
        }
    }
}

pub fn pred_die(mut pred_query: Query<&mut Predator>) {
    for mut pred in &mut pred_query.iter_mut().filter(|p| !p.dead) {
        if rand::random::<f32>() < pred.age as f32 / PRED_LIFESPAN * PRED_DIE_SUCCESS {
            pred.dead = true;
            fertilize(pred.idx_x, pred.idx_y);
        }
    }
}

pub fn pred_move(mut hunt_query: Query<(&mut Predator, &Transform)>, mut prey_query: Query<(&mut Prey, &Transform)>) {

    for (mut pred, pred_trans) in hunt_query.iter_mut().filter(|(p, _)| p.energy < 1. && !p.dead) {

        let closest_dead = prey_query
            .iter()
            .filter(|(p, _)| p.dead)
            .min_by(|(_, p_tran1), (_, p_tran2)| {
                let dist1 = pred_trans.translation.distance(p_tran1.translation);
                let dist2 = pred_trans.translation.distance(p_tran2.translation);
                dist1.partial_cmp(&dist2).unwrap()
            });

        
        let mut start_hunt: bool = true;

        match closest_dead {
            Some(m) => {
                if m.1.translation.distance(pred_trans.translation) < (2. * pred.speed as f32 * (CELL_SIZE + GRID_PADDING)) {  // if I can reach it in 2 cycles
                    let new_pos = goto(vec2(pred_trans.translation.x,pred_trans.translation.y), vec2(m.1.translation.x, m.1.translation.y), pred.speed, true);
                    
                    pred.idx_x = new_pos[0];
                    pred.idx_y = new_pos[1];
                    
                    start_hunt = false;
                }
            }
            None => println!("No dead prey"),
        }

        if start_hunt {

            let closest_alive = prey_query
                .iter_mut()
                .filter(|(p, _)| !p.dead)
                .min_by(|(_, p_tran1), (_, p_tran2)| {
                    let dist1 = pred_trans.translation.distance(p_tran1.translation);
                    let dist2 = pred_trans.translation.distance(p_tran2.translation);
                    dist1.partial_cmp(&dist2).unwrap()
                });

            match closest_alive {
                Some(mut m) => {
                    let prey_new_pos = goto(vec2(pred_trans.translation.x,pred_trans.translation.y), vec2(m.1.translation.x, m.1.translation.y), m.0.speed, false);
                    m.0.idx_x = prey_new_pos[0];
                    m.0.idx_y = prey_new_pos[1];

                    let pred_new_pos = goto(vec2(pred_trans.translation.x,pred_trans.translation.y), vec2(idx_to_pixel(m.0.idx_x as f32), idx_to_pixel(m.0.idx_y as f32)), pred.speed, true);
                    pred.idx_x = pred_new_pos[0];
                    pred.idx_y = pred_new_pos[1];
                }
                None => println!("No prey"),
            }
        }


    }
}

pub fn render_predator(mut predator_render_query: Query<(&mut Predator, &mut MeshMaterial2d<ColorMaterial>, &mut Transform)>, mut materials: ResMut<Assets<ColorMaterial>>) { 

    for (mut pred, mat,  mut tran) in &mut predator_render_query {
        
        if pred.idx_x < 0. {
            pred.idx_x = 0.;
        } else if pred.idx_x >= GRID_WIDTH as f32 {
            pred.idx_x = GRID_WIDTH as f32 - 1.;
        }
        if pred.idx_y < 0. {
            pred.idx_y = 0.;
        } else if pred.idx_y >= GRID_HEIGHT as f32 {
            pred.idx_y = GRID_HEIGHT as f32 - 1.;
        }

        tran.translation.x = idx_to_pixel(pred.idx_x as f32);
        tran.translation.y = idx_to_pixel(pred.idx_y as f32);

        if let Some(material) = materials.get_mut(mat.id()) {
            if pred.dead {
                material.color = COL_PRED_D;
            } else if pred.energy >= 1. {
                material.color = COL_PRED;
            } else { 
                material.color = COL_PRED_H;
            }
        }
    }
}


pub fn hunt(hunt_query: Query<(&Predator, &Transform)>, mut prey_query: Query<(&mut Prey, &Transform)>) {

    for (_pred, pred_trans) in hunt_query.iter().filter(|(p, _)| p.energy < 1. && !p.dead) {
        for (mut prey, prey_trans) in prey_query.iter_mut().filter(|(p, _)| !p.dead){
            if in_range(pred_trans, prey_trans, 1.5) {        
                if rand::random::<f32>()  < HUNT_SUCCESS {
                    prey.dead = true;
                }
            }
        }
    }
}


pub fn consume(mut commands: Commands, 
    mut hunt_query: Query<(&mut Predator, &Transform)>, 
    mut prey_query: Query<(Entity, &mut Prey, &Transform)>,
    mut plant_query: Query<(Entity, &mut Plant, &Transform)>,
    ) {

    for (mut pred, pred_trans) in hunt_query.iter_mut().filter(|(p, _)| p.energy < 1. && !p.dead) {
        for (prey_id, _prey, prey_trans) in prey_query.iter_mut().filter(|(_, p, _)| !p.dead) {
            if in_range(pred_trans, prey_trans, 1.5) {        
                pred.energy += 1.;
                commands.entity(prey_id).despawn();
                fertilize(pred.idx_x, pred.idx_y);
            }
        }
    }

    for (_prey_id, mut prey, prey_trans) in prey_query.iter_mut().filter(|(_, p, _)| p.energy < 1. && !p.dead) {
        for (plant_id, _plant, plant_trans) in plant_query.iter_mut().filter(|(_, p, _)| !p.dead) {
            if in_range(prey_trans, plant_trans, 1.5) {        
                prey.energy += 1.;
                commands.entity(plant_id).despawn();
                fertilize(prey.idx_x, prey.idx_y);
            }
        }
    }
}
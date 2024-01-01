use std::f32::consts::PI;
use std::io::{BufReader, Read, LineWriter, Write};
use std::fs::File;

use serde::{Serialize, Deserialize};

use bevy::{prelude::*, ecs::system::ResMut};
use bevy_egui::{egui, EguiContexts};

use bevy_rapier3d::prelude::*;

const WALL_HEIGHT: f32 = 5.;
const WALL_WIDTH: f32 = 1.2;
const VERTEX_SIDE: f32 = 1.2;
const SCALE: f32 = 0.5;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Maze {
    name: String,
    size: u32,
    encoding: String
}

impl Default for Maze {
    fn default() -> Self {
        Self {
            name: "Default Maze".to_string(),
            size: 16,
            encoding: "0x0".to_string()
        }
    }
}

#[derive(Resource)]
pub struct MazeConfig {
    pub cell_space: f32,
    pub maze_entities: Vec<Entity>,
    pub prev_mazes: Vec<Maze>,
    pub current_maze: usize,
    pub edit_maze: bool,
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self { 
            cell_space: 16., 
            prev_mazes: vec![Maze::default()],
            current_maze: 0,
            maze_entities: Vec::new() ,
            edit_maze: false
        }
    }
}

pub fn maze_ui (
    mut contexts: EguiContexts,
    mut maze_cfg: ResMut<MazeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    egui::Window::new("Maze").show( contexts.ctx_mut(), |ui| {
        if !maze_cfg.edit_maze {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::ComboBox::from_label("Previous Mazes")
                .selected_text( &maze_cfg.prev_mazes[maze_cfg.current_maze].name )
                .show_ui(ui, |ui| {
                    for (i, maze) in maze_cfg.prev_mazes.clone().iter().enumerate() {
                        let name = maze.name.clone();
                        if ui.selectable_value(&mut maze_cfg.current_maze, i, name).clicked() {
                            draw_maze(&mut maze_cfg, &mut meshes, &mut materials, &mut commands);
                        }
                    }
                });

                ui.label(format!("Maze size: {}", maze_cfg.prev_mazes[maze_cfg.current_maze].size));
            });

            if ui.button("New Maze").clicked() {
                maze_cfg.edit_maze = true;
                let new_maze: Maze = Maze {
                    name: "".to_string(),
                    size: 0,
                    encoding: "".to_string()
                };
                maze_cfg.prev_mazes.push(new_maze);
                maze_cfg.current_maze = maze_cfg.prev_mazes.len() - 1;
            };
        } else {
            let current_maze = maze_cfg.current_maze;
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut maze_cfg.prev_mazes[current_maze].name);
            });

            ui.horizontal(|ui| {
                ui.label("Size");
                ui.add(egui::DragValue::new(&mut maze_cfg.prev_mazes[current_maze].size));
            });

            ui.horizontal(|ui| {
                ui.label("Encoding");
                ui.text_edit_singleline(&mut maze_cfg.prev_mazes[current_maze].encoding);
            });

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    maze_cfg.edit_maze = false;

                    // Saving the newly created file
                    let previous_mazes_file = match File::create("./resources/previous_mazes.json") {
                        Ok(val) => val,
                        Err(_) => {
                            eprintln!("Failed to open previous mazes file");
                            return
                        } 
                    };

                    let mut writer = LineWriter::new(previous_mazes_file);
                    let json_str: String = match serde_json::to_string(&maze_cfg.prev_mazes) {
                        Ok(val) => val,
                        Err(_) => {
                            eprintln!("Failed to convert to string");
                            return;
                        }
                    };
                    match write!(writer, "{}", json_str) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("Failed to write to file");
                            return;
                        }
                    }

                    draw_maze(&mut maze_cfg, &mut meshes, &mut materials, &mut commands);
                };

                if ui.button("Cancel").clicked() {
                    maze_cfg.prev_mazes.pop();
                    maze_cfg.current_maze = maze_cfg.prev_mazes.len() - 1;
                    maze_cfg.edit_maze = false;

                    draw_maze(&mut maze_cfg, &mut meshes, &mut materials, &mut commands);
                };
            });
        }
    });
}

pub fn maze_setup(
    mut maze_cfg: ResMut<MazeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    let previous_mazes_file = match File::open("./resources/previous_mazes.json") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Failed to open previous mazes file");
            return
        } 
    };
    let mut reader = BufReader::new(previous_mazes_file);
    let mut json_str: String = String::new();
    match reader.read_to_string(&mut json_str) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Failed to read previous mazes file");
            return
        } 
    }

    maze_cfg.prev_mazes = match serde_json::from_str(&json_str) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Failed to deserialize previous mazes");
            return
        }
    };

    draw_maze(&mut maze_cfg, &mut meshes, &mut materials, &mut commands);
}

pub fn draw_maze(
    cfg: &mut MazeConfig,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    commands: &mut Commands
) {

    for ent in cfg.maze_entities.iter() {
        commands.entity(*ent).despawn();
    }
    cfg.maze_entities.clear();

    let current_maze = &cfg.prev_mazes[cfg.current_maze];
    let maze_size = cfg.cell_space * current_maze.size as f32 * SCALE;
    let scale= Vec3::splat(SCALE);
    let translation= Vec3::new(-maze_size / 2., 1., -maze_size / 2.);
    let origin = Transform::from_translation(translation).with_scale(scale);

    let start_marker = commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(
            2. * VERTEX_SIDE, 2. * WALL_HEIGHT, 2. * VERTEX_SIDE,
        ).into()),
        material: materials.add(Color::YELLOW_GREEN.into()),
        transform: origin,
        ..default()
    }).with_children(|parent| {

        // vertices
        let vertex_mesh = shape::Box::new(0.9 * VERTEX_SIDE, 0.9 * WALL_HEIGHT, 0.9 * VERTEX_SIDE);
        let vertex_mat = Color::WHITE;
        for i in 1..current_maze.size {
            for j in 1..current_maze.size {
                let new_vertex = parent.spawn(PbrBundle {
                    mesh: meshes.add(vertex_mesh.into()),
                    material: materials.add(vertex_mat.into()),
                    transform: Transform::from_xyz(i as f32 * cfg.cell_space, 0., j as f32 * cfg.cell_space),
                    ..default()
                }).id();
                cfg.maze_entities.push(new_vertex);
            }
        }

        // Outside walls
        let wall_length = cfg.cell_space * current_maze.size as f32;
        // x is north / south, z is east / west
        let wall_positions = [
            (wall_length, wall_length / 2., PI / 2.), // North Outside wall
            (wall_length / 2., wall_length, 0.), // East Outside wall
            (0., wall_length / 2., PI / 2.), // South Outside wall
            (wall_length / 2., 0., 0.), // West Outside wall
        ];
        let outer_wall_mesh = shape::Box::new(
            cfg.cell_space * current_maze.size as f32, 1.1 * WALL_HEIGHT, 1.1 * WALL_WIDTH
        );
        let outer_wall_mat = Color::WHITE;
        for i in 0..wall_positions.len() {
            let outer_wall_transform = Transform::from_xyz(wall_positions[i].0, 0., wall_positions[i].1).with_rotation(Quat::from_rotation_y(wall_positions[i].2));

            let outside_wall_collider = parent.spawn(Collider::cuboid(
                cfg.cell_space * current_maze.size as f32 / 2., WALL_HEIGHT/ 2., WALL_WIDTH / 2.
            ))
            .insert(TransformBundle::from(
                outer_wall_transform
            )).id();

            let new_outside_wall = parent.spawn(PbrBundle {
                mesh: meshes.add(outer_wall_mesh.into()),
                material: materials.add(outer_wall_mat.into()),
                transform: outer_wall_transform,
                ..default()
            }).id();
            cfg.maze_entities.push(new_outside_wall);
            cfg.maze_entities.push(outside_wall_collider);
        }

        let encoding_length = ((current_maze.size.pow(2) as f32) - (current_maze.size.pow(2) as f32 / 2.0).floor()) as u32;
        let mut encoding: Vec<u8> = Vec::new();
        let cell_center = cfg.cell_space / 2.;
        let n = current_maze.size;
        let inner_wall_width = WALL_WIDTH;

        let inner_wall_mesh = shape::Box::new(inner_wall_width, WALL_HEIGHT, cfg.cell_space);
        let inner_wall_mat = Color::RED;
                        
        for i in 0..encoding_length {
            let cell_encoding =  match current_maze.encoding.chars().nth((i + 2) as usize) {
                Some(val) => val,
                None => '\0'
            };

            let cell_value = match u8::from_str_radix(format!("0{}", cell_encoding).as_str(), 16) {
                Ok(val) => val,
                Err(_) => 0
            };

            // row = floor( 2 * index / size )
            // col = (index % (size / 2)) * 2 + (1 - size % 2) * ( row % 2 )

            let r = 2 * i / n;
            let c = ((i as f32 % (n as f32 / 2.)) * 2.) as u32 + (1 - n % 2) * (r % 2);

            let inner_wall_rotations = [ 0., PI / 2., 0., PI / 2. ];
            let inner_wall_translations = [ (cell_center, 0.), (0., cell_center), (-cell_center, 0.), (0., -cell_center) ];
            let inner_wall_encodings = [1, 2, 4, 8];
            for i in 0..4 {
                if cell_value & inner_wall_encodings[i] == inner_wall_encodings[i] {
                    let cell_wall_transform = Transform::from_xyz(
                        cell_center + r as f32 * cfg.cell_space + inner_wall_translations[i].0,
                        0.,
                        cell_center + c as f32 * cfg.cell_space + inner_wall_translations[i].1
                    ).with_rotation(Quat::from_rotation_y(inner_wall_rotations[i]));

                    let cell_wall_collider = parent.spawn(Collider::cuboid(
                    inner_wall_width / 2., WALL_HEIGHT / 2., cfg.cell_space / 2.
                    )).insert(
                        TransformBundle::from(cell_wall_transform)
                    ).id();

                    let cell_wall = parent.spawn(PbrBundle {
                        mesh: meshes.add(inner_wall_mesh.into()),
                        material: materials.add(inner_wall_mat.into()),
                        transform: cell_wall_transform,
                        ..default()
                    }).id();
                    cfg.maze_entities.push(cell_wall);
                    cfg.maze_entities.push(cell_wall_collider);
                }
            }

            encoding.push(cell_value);
        }
    }).id();
    cfg.maze_entities.push(start_marker);

}
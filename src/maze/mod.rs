use bevy::{prelude::*, ecs::system::ResMut};
use bevy_egui::{egui, EguiContexts};

#[derive(Resource)]
pub struct MazeConfig {
    pub maze_string: String,
    pub grid_size: u32,
    pub cell_space: f32,
    pub maze_entities: Vec<Entity>
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self { 
            maze_string: "0xfffff".to_string(), 
            grid_size: 16, 
            cell_space: 2., 
            maze_entities: Vec::new() 
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
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Maze string: ");
                ui.text_edit_singleline(&mut maze_cfg.maze_string);
            });

            ui.horizontal(|ui| {
                ui.label("Grid Size: ");
                ui.add(egui::DragValue::new(&mut maze_cfg.grid_size));
            });

            if ui.button("Build new maze").clicked() {
                let mut origin = Transform::from_xyz((maze_cfg.grid_size / 2) as f32 * -maze_cfg.cell_space , 0.4, (maze_cfg.grid_size / 2) as f32 * -maze_cfg.cell_space);
                draw_maze(&mut origin, &mut maze_cfg, &mut meshes, &mut materials, &mut commands);
            }
        });
    });
}

pub fn maze_setup(
    mut maze_cfg: ResMut<MazeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    let mut origin = Transform::from_xyz(0., 0.4, 0.);
    draw_maze(&mut origin, &mut maze_cfg, &mut meshes, &mut materials, &mut commands);
}

pub fn draw_maze(
    origin: &mut Transform,
    cfg: &mut MazeConfig,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    commands: &mut Commands
) {
    let vertex_width = 0.1;
    let vertex_height = 1.0;

    for ent in cfg.maze_entities.iter() {
        commands.entity(*ent).despawn();
    }
    cfg.maze_entities.clear();

    for i in 0..(cfg.grid_size + 1) {
        for j in 0..(cfg.grid_size + 1) {
            let new_vertex = commands.spawn(PbrBundle {
                mesh: meshes.add(shape::Box::new(vertex_width, vertex_height, vertex_width).into()),
                material: materials.add(Color::WHITE.into()),
                transform: origin.mul_transform(Transform::from_xyz(i as f32 * cfg.cell_space, 0., j as f32 * cfg.cell_space)),
                ..default()
            });
            cfg.maze_entities.push(new_vertex.id());
        }
    }

}
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_flycam::{PlayerPlugin, KeyBindings};

mod maze;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(PlayerPlugin)
        .insert_resource(KeyBindings {
            toggle_grab_cursor: KeyCode::C,
            ..default()
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });

    // Floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(5.0, 0.4, 5.).into()),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });

    // Origin Marker
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{ size: 1.0 })),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..default()
    });
}
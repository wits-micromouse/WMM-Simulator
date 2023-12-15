use bevy::{
    prelude::*,
    pbr::NotShadowCaster,
};
use bevy_egui::{egui, EguiPlugin, EguiContexts};
use bevy_flycam::{PlayerPlugin, KeyBindings};

use bevy_rapier3d::prelude::*;

mod maze;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(KeyBindings {
            toggle_grab_cursor: KeyCode::Tab,
            ..default()
        })
        .insert_resource(maze::MazeConfig::default())
        .add_systems(Startup, (setup, maze::maze_setup))
        .add_systems(Update, (perf_stats_ui, controls_ui, maze::maze_ui))
        .run();
}

fn perf_stats_ui(
    mut contexts: EguiContexts,
    time: Res<Time<bevy::prelude::Real>>,
) {
    egui::Window::new("Stats").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS: {:.2}", 1. / time.delta_seconds()));
        ui.label(format!("Frame time: {:.2} ms", time.delta_seconds() * 1000.));
    });
}

fn controls_ui(
    mut contexts: EguiContexts,
) {
    egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        ui.label("Toggle Cursor - tab");
        ui.label("Walk - wasd");
        ui.label("Look - mouse");
        ui.label("Fly up - space");
        ui.label("Fly down - left shift");
    });
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        ..default()
    });

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("99BADD").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(200.0)),
            ..default()
        },
        NotShadowCaster,
    ));

    // Floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(200.0, 0.4, 200.).into()),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });

    commands.spawn(
        Collider::cuboid(
            100.0, 0.4, 100.
        )
    );
}
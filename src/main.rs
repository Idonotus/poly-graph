mod simulation;
mod gui;

use simulation::shape::{Polyhedron, Face, Edge, Vertex};

use bevy::{
    prelude::*
};

// Define a "marker" component to mark the custom mesh. Marker components are often used in Bevy for
// filtering entities in queries with `With`, they're usually not queried directly since they don't
// contain information within them.
#[derive(Component)]
struct CustomUV;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube = &Polyhedron{
        faces: vec![
            F
        ]
    };

    // Create and save a handle to the mesh.
    let cube_mesh_handle: Handle<Mesh> = meshes.add(gui::meshgenerator::generate_mesh(cube));

    // Render the mesh with the custom texture, and add the marker.
    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            ..default()
        })),
        CustomUV,
    ));

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform));

    // Light up the scene.
    commands.spawn((PointLight::default(), camera_and_light_transform));

    // Text to describe the controls.
    commands.spawn((
        Text::new("Controls:\nSpace: Change UVs\nX/Y/Z: Rotate\nR: Reset orientation"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

// System to receive input from the user,
// check out examples/input/ for more examples about user input.
fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<CustomUV>>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
}
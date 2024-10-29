use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::{PrimaryWindow, WindowResolution},
};

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 640.0;
const TOP: f32 = HEIGHT / 2.0;
const BOTTOM: f32 = -(HEIGHT / 2.0);
const RIGHT: f32 = WIDTH / 2.0;
const LEFT: f32 = -(WIDTH / 2.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<MyWorldCoords>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_lines, mouse_world_coordinates, close_on_esc),
        )
        .run();
}

#[derive(Component, Default)]
struct Line {
    transform: Transform,
    mesh: Rectangle,
}

impl Line {
    fn from_pts(start: (f32, f32), end: (f32, f32), stroke: f32) -> Self {
        // x and y component distances from the start to end point.
        let x = end.0 - start.0;
        let y = end.1 - start.1;
        // length via distance formula
        let length = ((x * x) + (y * y)).sqrt();
        //transform accounting for the transform positiong being relative to the center of the object in bevy
        let mut transform = Transform::from_xyz(start.0 + (x / 2.0), start.1 + (y / 2.0), 0.0);

        // angle of line based on offset fom x-axis
        let theta = (y / x).atan();
        transform.rotation = Quat::from_rotation_z(theta);

        Self {
            transform,
            mesh: Rectangle::new(length, stroke),
        }
    }
}

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, MainCamera));
}

fn mouse_world_coordinates(
    mut mouse_coords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    // transform mouse pixel coordinates to world coordinates to play nicely with transforms
    mouse_coords.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        .unwrap_or_default();
}

fn update_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_world_coords: ResMut<MyWorldCoords>,
    mut query_curr_lines: Query<Entity, (With<Transform>, Without<Camera>)>,
) {
    // a bit hacky to delete the old lines and spawn new ones
    // would need to change if dealing with more lines
    for id in &mut query_curr_lines {
        commands.entity(id).despawn();
    }

    let line_to_center = Line::from_pts((LEFT, TOP), (0.0, 0.0), 5.0);
    let line_to_mouse = Line::from_pts(
        (LEFT, TOP),
        (mouse_world_coords.0.x, mouse_world_coords.0.y),
        5.0,
    );
    // the generation of the "subtraction" line doesnt reflect vector subtration just due to how vectors and line generation works in bevy
    let sub_line = Line::from_pts(
        (0.0, 0.0),
        (mouse_world_coords.0.x, mouse_world_coords.0.y),
        5.0,
    );

    // the pictoral result is still the same
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(sub_line.mesh)),
        material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
        transform: sub_line.transform,
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(line_to_center.mesh)),
        material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
        transform: line_to_center.transform,
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(line_to_mouse.mesh)),
        material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
        transform: line_to_mouse.transform,
        ..default()
    });
}

// Simple function to close window when esc is pressed
fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window_ent, _focus) in &focused_windows {
        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window_ent).despawn();
        }
    }
}

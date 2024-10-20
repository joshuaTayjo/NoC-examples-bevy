use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
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
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

#[derive(Component)]
struct Line(Segment2d);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle { ..default() });

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle {
            half_size: Vec2::new(20.0, 1.0),
        })),
        material: materials.add(Color::srgb(0.8, 0.0, 0.2)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
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

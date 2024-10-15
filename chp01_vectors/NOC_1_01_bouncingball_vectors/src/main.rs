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
        .add_systems(Update, (ball_move, close_on_esc))
        .run();
}

#[derive(Component)]
struct Ball {
    position: Vec2,
    speed: Vec2,
}

impl Ball {
    fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            speed: Vec2::new(2.5, 2.0),
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle { ..default() });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(20.0))),
            material: materials.add(Color::srgb(0.8, 0.01, 0.4)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Ball::new(),
    ));
}
fn ball_move(mut query: Query<(&mut Ball, &mut Transform)>) {
    let (mut ball, mut transform) = query.single_mut();
    let speed = ball.speed;
    ball.position += speed;

    if ball.position.x > RIGHT || ball.position.x < LEFT {
        ball.speed.x = -ball.speed.x
    }
    if ball.position.y > TOP || ball.position.y < BOTTOM {
        ball.speed.y = -ball.speed.y
    }
    transform.translation = ball.position.extend(0.0);
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

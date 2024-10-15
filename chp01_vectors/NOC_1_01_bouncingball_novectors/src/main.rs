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
        .add_plugins(DefaultPlugins.set({
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WIDTH, HEIGHT),
                    ..default()
                }),
                ..default()
            }
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (ball_move, close_on_esc))
        .run();
}

#[derive(Component)]
struct Ball {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
}

impl Ball {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            x_speed: 2.5,
            y_speed: 2.0,
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

    ball.x += ball.x_speed;
    ball.y += ball.y_speed;
    if ball.x > RIGHT || ball.x < LEFT {
        ball.x_speed = -ball.x_speed
    }
    if ball.y > TOP || ball.y < BOTTOM {
        ball.y_speed = -ball.y_speed
    }

    transform.translation = (ball.x, ball.y, 0.0).into();
}

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

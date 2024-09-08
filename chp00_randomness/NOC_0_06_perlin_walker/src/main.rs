use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use noise::{self, NoiseFn};

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 640.0;

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
        .add_systems(Update, (walk, close_on_esc))
        .run();
}

/// Represents transform and noise "time" values for a single Walker
#[derive(Component)]
struct Walker {
    // Could do more to optimize the memory use but in this case doesn't matter that much
    x: f64,
    y: f64,
    tx: f64,
    ty: f64,
}

impl Walker {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            tx: 100.0,
            ty: 0.0,
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
            mesh: Mesh2dHandle(meshes.add(Circle::new(8.0))),
            material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Walker::new(),
    ));
}

fn walk(mut query: Query<(&mut Walker, &mut Transform)>) {
    let (mut walker, mut transform) = query.single_mut();
    // increment "time" values for noise function
    walker.tx += 0.005;
    walker.ty += 0.005;
    // seed must be the same every run for consistent and useful values
    let noise = noise::Perlin::new(1);

    // ouptut range for this version of Perlin noise is (-1,1) so must map to screen size
    walker.x = map(
        noise.get([walker.tx]),
        -1.0,
        1.0,
        -(WIDTH / 2.0) as f64,
        (WIDTH / 2.0) as f64,
    );
    walker.y = map(
        noise.get([walker.ty]),
        -1.0,
        1.0,
        -(HEIGHT / 2.0) as f64,
        (HEIGHT / 2.0) as f64,
    );

    transform.translation = (walker.x as f32, walker.y as f32, 0.0).into();
}

/// Primitive function to map floats from one range to floats in another
fn map(n: f64, input_start: f64, input_end: f64, output_start: f64, output_end: f64) -> f64 {
    ((output_end - output_start) / (input_end - input_start)) * (n - input_start) + output_start
}

/// Simple function to close the window on user pressing "escape"
fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, _focus) in &focused_windows {
        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

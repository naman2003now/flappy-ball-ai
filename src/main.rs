use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.8);
const BALL_COLOR: Color = Color::rgb(0.8, 0.1, 0.2);

const BALL_RADIUS: f32 = 25.0;
const GRAVITY: f32 = 2000.0;
const JUMP_VELOCITY: f32 = 700.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (physics, jump))
        .run();
}

#[derive(Component)]
struct Ball {
    velocity: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: BALL_RADIUS,
            })),
            material: materials.add(BALL_COLOR),
            transform: Transform::from_xyz(-500.0, 0.0, 0.0),
            ..default()
        },
        Ball { velocity: 0.0 },
    ));
}

// Physics with delta time
fn physics(time: Res<Time>, mut ball_query: Query<(&mut Ball, &mut Transform)>) {
    for (mut ball, mut transform) in ball_query.iter_mut() {
        ball.velocity -= GRAVITY * time.delta_seconds();
        transform.translation.y += ball.velocity * time.delta_seconds();
    }
}

// Jump when space pressed
fn jump(mut ball_query: Query<&mut Ball>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    for mut ball in ball_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            ball.velocity = JUMP_VELOCITY;
        }
    }
}

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.8);
const BALL_COLOR: Color = Color::rgb(0.8, 0.1, 0.2);

const BALL_RADIUS: f32 = 25.0;
const GRAVITY: f32 = 2000.0;
const JUMP_VELOCITY: f32 = 700.0;

const PIPE_GAP: f32 = 200.0;
const PIPE_WIDTH: f32 = 50.0;
const PIPE_SPEED: f32 = 200.0;
const PIPE_HEIGHT_MIN: f32 = -250.0;
const PIPE_HEIGHT_MAX: f32 = 250.0;
const PIPE_SPAWN_INTERVAL: f32 = 2.0;
const PIPE_COLOR: Color = Color::rgb(0.1, 0.1, 0.2);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (physics, jump, spawn_pipes, move_pipes))
        .run();
}

#[derive(Component)]
struct Ball {
    velocity: f32,
}

#[derive(Component)]
struct Pipe;

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

fn spawn_pipes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut pipe_timer: Local<f32>,
) {
    *pipe_timer -= time.delta_seconds();
    if *pipe_timer <= 0.0 {
        let pipe_height =
            PIPE_HEIGHT_MIN + (PIPE_HEIGHT_MAX - PIPE_HEIGHT_MIN) * rand::random::<f32>();
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(PIPE_WIDTH, 2000.0))),
                material: materials.add(PIPE_COLOR),
                transform: Transform::from_xyz(
                    1500.0,
                    pipe_height + 1000.0 + (PIPE_GAP / 2.0),
                    0.0,
                ),
                ..default()
            },
            Pipe,
        ));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(PIPE_WIDTH, 2000.0))),
                material: materials.add(PIPE_COLOR),
                transform: Transform::from_xyz(
                    1500.0,
                    pipe_height - 1000.0 - (PIPE_GAP / 2.0),
                    0.0,
                ),
                ..default()
            },
            Pipe,
        ));
        *pipe_timer = PIPE_SPAWN_INTERVAL;
    }
}

fn physics(time: Res<Time>, mut ball_query: Query<(&mut Ball, &mut Transform)>) {
    for (mut ball, mut transform) in ball_query.iter_mut() {
        ball.velocity -= GRAVITY * time.delta_seconds();
        transform.translation.y += ball.velocity * time.delta_seconds();
    }
}

fn move_pipes(time: Res<Time>, mut pipe_query: Query<(&mut Pipe, &mut Transform)>) {
    for (_, mut transform) in pipe_query.iter_mut() {
        transform.translation.x -= PIPE_SPEED * time.delta_seconds();
    }
}

fn jump(mut ball_query: Query<&mut Ball>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    for mut ball in ball_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            ball.velocity = JUMP_VELOCITY;
        }
    }
}

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const BALL_COLOR: Color = Color::rgb(0.8, 0.1, 0.2);
const BALL_RADIUS: f32 = 25.0;
const GRAVITY: f32 = 2000.0;
const JUMP_VELOCITY: f32 = 700.0;

#[derive(Component)]
pub struct Player {
    velocity: f32,
}

fn physics(time: Res<Time>, mut ball_query: Query<(&mut Player, &mut Transform)>) {
    for (mut ball, mut transform) in ball_query.iter_mut() {
        ball.velocity -= GRAVITY * time.delta_seconds();
        transform.translation.y += ball.velocity * time.delta_seconds();
    }
}

fn spawn(
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
        Player { velocity: 0.0 },
    ));
}

fn jump(mut player_query: Query<&mut Player>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    for mut ball in player_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            ball.velocity = JUMP_VELOCITY;
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (physics, jump));
    }
}

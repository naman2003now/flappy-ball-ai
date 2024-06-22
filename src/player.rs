use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::pipe::Pipe;

const BALL_COLOR: Color = Color::rgb(0.8, 0.1, 0.2);
const BALL_RADIUS: f32 = 25.0;
const GRAVITY: f32 = 2000.0;
const JUMP_VELOCITY: f32 = 700.0;

#[derive(Component)]
pub struct Player {
    velocity: f32,
    is_dead: bool,
    position: f32,
}

fn physics(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        player.velocity -= GRAVITY * time.delta_seconds();
        player.position += player.velocity * time.delta_seconds();
        transform.translation.y = player.position;
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
        Player {
            velocity: 0.0,
            is_dead: false,
            position: 0.0,
        },
    ));
}

fn jump(mut player_query: Query<&mut Player>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    for mut ball in player_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            ball.velocity = JUMP_VELOCITY;
        }
    }
}

fn death(mut player_query: Query<&mut Player>, pipe_query: Query<&Pipe>) {
    for mut player in player_query.iter_mut() {
        if player.position >= 350.0 || player.position <= -350.0 {
            player.is_dead = true;
            continue;
        }
        for pipe in pipe_query.iter() {
            if (pipe.position + 500.0).abs() >= 45.0 {
                continue;
            }
            if (pipe.height - player.position).abs() < 80.0 {
                continue;
            }
            player.is_dead = true;
        }
    }
}

fn game_over(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    pipe_query: Query<Entity, With<Pipe>>,
) {
    let mut game_over = true;
    for player in player_query.iter() {
        if !player.is_dead {
            game_over = false;
        }
    }
    if !game_over {
        return;
    }
    for entity in pipe_query.iter() {
        commands.entity(entity).despawn();
    }
    for mut player in player_query.iter_mut() {
        player.is_dead = false;
        player.position = 0.0;
        player.velocity = 0.0;
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (physics, jump, death, game_over));
    }
}

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::genetics::Brain;
use crate::pipe::Pipe;

const BALL_COLOR: Color = Color::rgba(0.8, 0.1, 0.2, 0.3);
const BEST_COLOR: Color = Color::rgba(0.2, 0.8, 0.2, 1.0);
const BALL_RADIUS: f32 = 25.0;
const GRAVITY: f32 = 4000.0;
const JUMP_VELOCITY: f32 = 1400.0;

#[derive(Component)]
pub struct Player {
    velocity: f32,
    is_dead: bool,
    position: f32,
    brain: Brain,
    score: f32,
}

fn physics(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        if player.is_dead {
            transform.translation.y = 2000.0;
            continue;
        }
        player.velocity -= GRAVITY * time.delta_seconds();
        player.position += player.velocity * time.delta_seconds();
        transform.translation.y = player.position;
        player.score += time.delta_seconds();
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle {
        radius: BALL_RADIUS,
    });
    let material = materials.add(BALL_COLOR);
    commands.spawn(Camera2dBundle::default());
    for _ in 0..1600 {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh.clone()),
                material: material.clone(),
                transform: Transform::from_xyz(-500.0, 0.0, 0.0),
                ..default()
            },
            Player {
                velocity: 0.0,
                is_dead: false,
                position: 0.0,
                score: 0.0,
                brain: Brain::new(),
            },
        ));
    }
}

fn jump(mut player_query: Query<&mut Player>, pipe_query: Query<&Pipe>) {
    let mut closest_pipe = None;
    let mut closest_distance = f32::INFINITY;
    for pipe in pipe_query.iter() {
        let distance = pipe.position + 500.0;
        if distance < 0.0 {
            continue;
        }
        if distance < closest_distance {
            closest_pipe = Some(pipe);
            closest_distance = distance;
        }
    }
    for mut ball in player_query.iter_mut() {
        if ball.is_dead {
            continue;
        }
        if closest_pipe.is_none() {
            ball.position = 0.0;
            ball.velocity = 0.0;
            continue;
        }
        let inputs = [
            ball.position,
            ball.velocity,
            closest_distance + 500.0,
            closest_pipe.unwrap().height - ball.position,
        ];
        let outputs = ball.brain.think(inputs);
        if outputs[0] > outputs[1] {
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
    player_query: Query<&Player>,
    pipe_query: Query<Entity, With<Pipe>>,
    player_entity_query: Query<Entity, With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
    let mut top_40_players = player_query.iter().collect::<Vec<_>>();
    top_40_players.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    let mut count = 0;
    for (_, player) in top_40_players.iter().enumerate().take(80) {
        for child in player.brain.generate_n_child(20) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle {
                        radius: BALL_RADIUS,
                    })),
                    material: materials.add(if count < 0 { BEST_COLOR } else { BALL_COLOR }),
                    transform: Transform::from_xyz(-500.0, 0.0, 0.0),
                    ..default()
                },
                Player {
                    velocity: 0.0,
                    is_dead: false,
                    position: 0.0,
                    score: 0.0,
                    brain: child,
                },
            ));
        }
        count += 1;
    }
    for _ in 0..100 {
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
                score: 0.0,
                brain: Brain::new(),
            },
        ));
    }
    for i in 1..40 {
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
                score: 0.0,
                brain: top_40_players[i].brain.clone(),
            },
        ));
    }
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: BALL_RADIUS,
            })),
            material: materials.add(BEST_COLOR),
            transform: Transform::from_xyz(-500.0, 0.0, 0.0),
            ..default()
        },
        Player {
            velocity: 0.0,
            is_dead: false,
            position: 0.0,
            score: 0.0,
            brain: top_40_players[0].brain.clone(),
        },
    ));
    for player in player_entity_query.iter() {
        commands.entity(player).despawn();
    }
    for pipe in pipe_query.iter() {
        commands.entity(pipe).despawn();
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (physics, jump, death, game_over));
    }
}

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const PIPE_GAP: f32 = 200.0;
const PIPE_WIDTH: f32 = 50.0;
const PIPE_SPEED: f32 = 400.0;
const PIPE_HEIGHT_MIN: f32 = -250.0;
const PIPE_HEIGHT_MAX: f32 = 250.0;
const PIPE_SPAWN_INTERVAL: f32 = 2.0;
const PIPE_COLOR: Color = Color::rgb(0.1, 0.1, 0.2);

#[derive(Component)]
pub struct Pipe {
    pub height: f32,
    pub position: f32,
}

fn spawn(
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
            Pipe {
                height: pipe_height,
                position: 1500.0,
            },
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
            Pipe {
                height: pipe_height,
                position: 1500.0,
            },
        ));
        *pipe_timer = PIPE_SPAWN_INTERVAL;
    }
}

fn move_pipes(time: Res<Time>, mut query: Query<(&mut Pipe, &mut Transform)>) {
    for (mut pipe, mut transform) in query.iter_mut() {
        transform.translation.x -= PIPE_SPEED * time.delta_seconds();
        pipe.position = transform.translation.x;
    }
}

pub struct PipePlugin;
impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn, move_pipes));
    }
}

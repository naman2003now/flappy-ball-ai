mod genetics;
mod pipe;
mod player;

use bevy::prelude::*;
use pipe::PipePlugin;
use player::PlayerPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.8);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((DefaultPlugins, PlayerPlugin, PipePlugin))
        .run();
}

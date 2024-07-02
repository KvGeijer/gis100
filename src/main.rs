use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;

use crate::node::spawn_node;

mod asset_loader;
mod camera;
mod node;
mod spring_layout;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // My own systems
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(PostStartup, spawn_example_graph)
        .run();
}

fn spawn_example_graph(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const OFFSET_X: f32 = 2.5;
    const OFFSET_Y: f32 = 2.5;

    spawn_node(&mut commands, &mut meshes, &mut materials, 0., 0.);

    spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        OFFSET_X,
        OFFSET_Y,
    );
    spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        OFFSET_X,
        -OFFSET_Y,
    );
    spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        -OFFSET_X,
        OFFSET_Y,
    );
    spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        -OFFSET_X,
        -OFFSET_Y,
    );
}

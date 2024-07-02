use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use edge::{spawn_edge, EdgePlugin};

use crate::node::spawn_node;

mod asset_loader;
mod camera;
mod edge;
mod node;
mod spring_layout;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // My own systems
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(EdgePlugin)
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

    let a = spawn_node(&mut commands, &mut meshes, &mut materials, 0., 0.);

    let b = spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        OFFSET_X,
        OFFSET_Y,
    );
    let c = spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        OFFSET_X,
        -OFFSET_Y,
    );
    let d = spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        -OFFSET_X,
        OFFSET_Y,
    );
    let e = spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        -OFFSET_X,
        -OFFSET_Y,
    );

    spawn_edge(&mut commands, a, b);
    spawn_edge(&mut commands, b, c);
    spawn_edge(&mut commands, c, e);
    spawn_edge(&mut commands, e, d);
    spawn_edge(&mut commands, d, b);
}

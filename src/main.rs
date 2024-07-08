use std::collections::HashSet;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use camera::CameraPlugin;
use edge::{spawn_edge, EdgePlugin};
use node::{NodeColor, NodePlugin};
use rand::Rng;
use selected::SelectedPlugin;
use spring_layout::SpringLayoutPlugin;

use crate::node::spawn_node;

mod asset_loader;
mod camera;
mod edge;
mod node;
mod selected;
mod spring_layout;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 3rd party plugins
        .add_plugins(DefaultPickingPlugins)
        // My own systems
        .add_plugins(NodePlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(EdgePlugin)
        .add_plugins(SelectedPlugin)
        // .add_systems(Startup, spawn_example_graph)
        .add_systems(Startup, spawn_random_graph)
        .add_plugins(SpringLayoutPlugin)
        .run();
}

fn spawn_example_graph(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    color: Res<NodeColor>,
) {
    const OFFSET_X: f32 = 2.5;
    const OFFSET_Y: f32 = 2.5;

    let a = spawn_node(&mut commands, &mut meshes, &mut materials, &color, 0., 0.);

    let b = spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        &color,
        OFFSET_X,
        OFFSET_Y,
    );
    let c = spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        &color,
        OFFSET_X,
        -OFFSET_Y,
    );
    let d = spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        &color,
        -OFFSET_X,
        OFFSET_Y,
    );
    let e = spawn_node(
        &mut commands,
        &mut meshes,
        &mut materials,
        &color,
        -OFFSET_X,
        -OFFSET_Y,
    );

    spawn_edge(&mut commands, a, b);
    spawn_edge(&mut commands, b, c);
    spawn_edge(&mut commands, c, e);
    spawn_edge(&mut commands, e, d);
    spawn_edge(&mut commands, d, a);
    spawn_edge(&mut commands, c, a);
}

fn spawn_random_graph(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    color: Res<NodeColor>,
) {
    const NODES: usize = 8;
    const EDGES: usize = 10;

    const XMIN: f32 = -16.0;
    const XMAX: f32 = 16.0;
    const YMIN: f32 = -10.0;
    const YMAX: f32 = 10.0;

    let mut rng = rand::thread_rng();

    let nodes: Vec<Entity> = (0..NODES)
        .map(|_| {
            spawn_node(
                &mut commands,
                &mut meshes,
                &mut materials,
                &color,
                rng.gen_range(XMIN..=XMAX),
                rng.gen_range(YMIN..=YMAX),
            )
        })
        .collect();

    let mut edges: HashSet<(Entity, Entity)> = HashSet::new();

    for (left, right) in nodes.iter().cloned().zip(nodes.iter().skip(1).cloned()) {
        spawn_edge(&mut commands, left, right);
        edges.insert((left, right));
    }

    while edges.len() < EDGES {
        let left = nodes[rng.gen_range(0..NODES)];
        let right = nodes[rng.gen_range(0..NODES)];

        if !(edges.contains(&(left, right)) || edges.contains(&(right, left))) {
            spawn_edge(&mut commands, left, right);
            edges.insert((left, right));
        }
    }
}

use asset_loader::{AssetLoaderPlugin, SceneAssets};
use bevy::prelude::*;
use camera::CameraPlugin;

mod asset_loader;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // My own systems and plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, spawn_example_graph)
        .run();
}

fn spawn_example_graph(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    const OFFSET_X: f32 = 2.5;
    const OFFSET_Y: f32 = 2.5;

    // Spawn four circles in a 2x2 grid
    commands.spawn((SceneBundle {
        scene: scene_assets.node.clone(),
        transform: Transform::from_xyz(OFFSET_X, OFFSET_Y, 0.),
        ..default()
    },));
    commands.spawn((SceneBundle {
        scene: scene_assets.node.clone(),
        transform: Transform::from_xyz(OFFSET_X, -OFFSET_Y, 0.),
        ..default()
    },));
    commands.spawn((SceneBundle {
        scene: scene_assets.node.clone(),
        transform: Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
        ..default()
    },));
    commands.spawn((SceneBundle {
        scene: scene_assets.node.clone(),
        transform: Transform::from_xyz(-OFFSET_X, -OFFSET_Y, 0.),
        ..default()
    },));
}

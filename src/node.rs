use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub const NODE_RADIUS: f32 = 1.0;

#[derive(Component)]
pub struct NodeMarker;

#[derive(Bundle)]
pub struct NodeBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    marker: NodeMarker,
}

pub fn spawn_node(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
) -> Entity {
    info!("Spawning node at {x} {y}");
    let mesh = MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle {
            radius: NODE_RADIUS,
        })),
        material: materials.add(Color::hsl(280., 0.25, 0.7)),
        transform: Transform::from_xyz(x, y, 0.0),
        ..default()
    };
    commands
        .spawn(NodeBundle {
            mesh,
            marker: NodeMarker,
        })
        .id()
}

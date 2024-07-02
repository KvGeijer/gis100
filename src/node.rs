use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Bundle)]
pub struct NodeBundle {
    scene: MaterialMesh2dBundle<ColorMaterial>,
}

pub fn spawn_node(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
) {
    commands.spawn((NodeBundle {
        scene: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 1.0 })),
            material: materials.add(Color::hsl(280., 0.25, 0.7)),
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
    },));
}

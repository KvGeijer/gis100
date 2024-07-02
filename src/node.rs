use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub const NODE_RADIUS: f32 = 1.0;

#[derive(Bundle)]
pub struct NodeBundle {
    scene: NodeMesh,
}

#[derive(Component)]
pub struct NodeMesh {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

pub fn spawn_node(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
) -> Entity {
    commands
        .spawn(NodeMesh {
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: NODE_RADIUS,
                })),
                material: materials.add(Color::hsl(280., 0.25, 0.7)),
                transform: Transform::from_xyz(x, y, 1.0),
                ..default()
            },
        })
        .id()
}

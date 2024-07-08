use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
    let mesh = MaterialMesh2dBundle {
        mesh: meshes
            .add(Circle {
                radius: NODE_RADIUS,
            })
            .into(),
        material: materials.add(Color::srgb(0.77, 0.67, 0.83)),
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

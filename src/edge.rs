use std::collections::LinkedList;

use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Bundle)]
pub struct EdgeBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    edge: EdgeComponent,
}

#[derive(Component)]
pub struct EdgeComponent {
    left: Entity,
    right: Entity,
    // directed: bool,
}

pub fn spawn_edge(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    left: Entity,
    right: Entity,
) {
    let line_mesh = Line {
        from: Vec3::splat(0.0),
        to: Vec3::splat(3.0),
    };
    commands.spawn(EdgeBundle {
        mesh: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(line_mesh)),
            material: materials.add(Color::hsl(200.0, 0.25, 0.75)),
            ..default()
        },
        edge: EdgeComponent { left, right },
    });
}

#[derive(Debug, Clone)]
pub struct Line {
    from: Vec3,
    to: Vec3,
}

impl From<Line> for Mesh {
    fn from(value: Line) -> Self {
        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![value.from, value.to])
    }
}

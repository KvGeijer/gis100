use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use drag::DragPlugin;
use selected::SelectedPlugin;

mod drag;
mod events;
mod selected;

use events::{NodeClickEvent, NodeDragEndEvent, NodeDragEvent, NodeDragStartEvent};

pub use drag::Dragged;

pub const NODE_RADIUS: f32 = 1.0;

pub struct NodePlugin;
impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NodeClickEvent>()
            .add_event::<NodeDragStartEvent>()
            .add_event::<NodeDragEvent>()
            .add_event::<NodeDragEndEvent>()
            .insert_resource(NodeColor {
                color: Color::srgb(0.77, 0.74, 0.83),
            })
            .add_plugins(SelectedPlugin)
            .add_plugins(DragPlugin)
            ;
    }
}

#[derive(Resource, Clone, Debug)]
pub struct NodeColor {
    pub color: Color,
}

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
    color: &Res<NodeColor>,
    x: f32,
    y: f32,
) -> Entity {
    let mesh = MaterialMesh2dBundle {
        mesh: meshes
            .add(Circle {
                radius: NODE_RADIUS,
            })
            .into(),
        material: materials.add(color.color.clone()),
        transform: Transform::from_xyz(x, y, 0.0),
        ..default()
    };
    commands
        .spawn((
            NodeBundle {
                mesh,
                marker: NodeMarker,
            },
            On::<Pointer<Click>>::send_event::<NodeClickEvent>(),
            On::<Pointer<DragStart>>::send_event::<NodeDragStartEvent>(),
            On::<Pointer<Drag>>::send_event::<NodeDragEvent>(),
            On::<Pointer<DragEnd>>::send_event::<NodeDragEndEvent>(),
        ))
        .id()
}

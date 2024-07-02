use bevy::prelude::*;

use crate::node::{NodeMarker, NODE_RADIUS};

pub struct EdgePlugin;

impl Plugin for EdgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_edges);
    }
}

#[derive(Component)]
pub struct Edge {
    left: Entity,
    right: Entity,
    // directed: bool,
}

/// Spawns an edge entity between two node entities
pub fn spawn_edge(commands: &mut Commands, left: Entity, right: Entity) {
    commands.spawn(Edge { left, right });
}

fn draw_edges(mut gizmos: Gizmos, edges: Query<&Edge>, nodes: Query<&Transform, With<NodeMarker>>) {
    for Edge { left, right } in edges.iter() {
        let left_v3 = nodes
            .get(*left)
            .expect("Could not find node neighbor of edge.")
            .translation;
        let right_v3 = nodes
            .get(*right)
            .expect("Could not find node neighbor of edge.")
            .translation;

        let lr = right_v3 - left_v3;
        if lr.length() > 2.0 * NODE_RADIUS {
            // Large enough to draw
            let lr_dir = lr.normalize();

            gizmos.line_2d(
                (left_v3 + lr_dir).truncate(),
                (right_v3 - lr_dir).truncate(),
                Color::ORANGE_RED,
            )
        }
    }
}

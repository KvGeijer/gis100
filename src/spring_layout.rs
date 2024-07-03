use bevy::prelude::*;

use crate::{
    edge::{draw_edges, Edge},
    node::{NodeMarker, NODE_RADIUS},
    spawn_example_graph,
};

const CENTER_ATTRACTION: f32 = 0.1;
const REPULSION_CONSTANT: f32 = 2.0;
const DESIRED_DISTANCE: f32 = 4.0 * NODE_RADIUS;
const SPRING_CONSTANT: f32 = 100.0;
const FORCE_DAMPENING: f32 = 0.1;
const MAX_MOVE: f32 = 4.0 * NODE_RADIUS;
const MAX_MOVE_SQ: f32 = MAX_MOVE * MAX_MOVE;

#[derive(Component)]
struct SpringForce {
    value: Vec3,
}

impl SpringForce {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

pub struct SpringLayoutPlugin;

impl Plugin for SpringLayoutPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Conditionally run these depending on a resource
        app.add_systems(
            Update,
            (
                spring_update_force,
                spring_update_position.before(draw_edges),
            )
                .chain(),
        )
        .add_systems(PostStartup, apply_spring_layout.after(spawn_example_graph));
    }
}

pub fn apply_spring_layout(mut commands: Commands, mut query: Query<Entity, With<NodeMarker>>) {
    for node in query.iter_mut() {
        commands.entity(node).insert(SpringForce::new(Vec3::ZERO));
    }
}

fn spring_update_force(
    mut self_query: Query<(&mut SpringForce, &Transform), With<NodeMarker>>,
    other_query: Query<&Transform, With<NodeMarker>>,
    edge_query: Query<&Edge>,
) {
    for (mut force, self_transform) in self_query.iter_mut() {
        // Apply similar pull towards the center
        let len_sq = self_transform.translation.length_squared();
        if len_sq > 0.0 {
            force.value = -CENTER_ATTRACTION * self_transform.translation.normalize_or_zero();
        }

        // Apply repulsive force between nodes
        for other_transform in other_query.iter() {
            let pos_diff = other_transform.translation - self_transform.translation;
            let len_sq = pos_diff.length_squared();
            if len_sq > 0.0 {
                // Can remove normalize to change from electrical to gravitational pull
                force.value -=
                    pos_diff.normalize() / pos_diff.length_squared() * REPULSION_CONSTANT;
            }
        }
    }

    // Apply attraction force between connected nodes
    for edge in edge_query.iter() {
        for [self_id, other_id] in [[edge.left, edge.right], [edge.right, edge.left]] {
            let (mut self_force, self_transform) = self_query
                .get_mut(self_id)
                .expect("Could not get edge node for spring force");

            let other_transform = other_query
                .get(other_id)
                .expect("Could not get edge node for spring force");

            let pos_diff = other_transform.translation - self_transform.translation;
            let dist_offset = pos_diff.length() - DESIRED_DISTANCE;
            self_force.value += dist_offset * pos_diff.normalize() * SPRING_CONSTANT;
        }
    }
}

fn spring_update_position(mut query: Query<(&SpringForce, &mut Transform)>, time: Res<Time>) {
    for (force, mut transform) in query.iter_mut() {
        let vel = force.value * time.delta_seconds() * FORCE_DAMPENING;
        let restricted_vel = if vel.length_squared() > MAX_MOVE_SQ {
            vel.normalize() * MAX_MOVE
        } else {
            vel
        };

        transform.translation += restricted_vel;
    }
}

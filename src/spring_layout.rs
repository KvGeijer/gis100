use bevy::prelude::*;

use crate::{
    edge::{draw_edges, Edge},
    node::{NodeMarker, NODE_RADIUS},
    spawn_example_graph,
};

const GRAVITY_CONSTANT: f32 = 0.982;
const REPULSION_CONSTANT: f32 = 0.5;
const DESIRED_DISTANCE: f32 = 3.0 * NODE_RADIUS;
const SPRING_CONSTANT: f32 = 1000.0;

#[derive(Component)]
struct Velocity {
    value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component)]
struct Acceleration {
    value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
struct SpringMomentumBundle {
    velocity: Velocity,
    acceleration: Acceleration,
}

pub struct SpringLayoutPlugin;

impl Plugin for SpringLayoutPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Conditionally run these depending on a resource
        app.add_systems(
            Update,
            (
                spring_update_acceleration,
                spring_update_velocity,
                spring_update_position.before(draw_edges),
            ),
        )
        .add_systems(PostStartup, apply_spring_layout.after(spawn_example_graph));
    }
}

pub fn apply_spring_layout(mut commands: Commands, mut query: Query<Entity, With<NodeMarker>>) {
    for node in query.iter_mut() {
        commands
            .entity(node)
            .insert((Acceleration::new(Vec3::ZERO), Velocity::new(Vec3::ZERO)));
    }
}

fn spring_update_acceleration(
    mut self_query: Query<(&mut Acceleration, &Transform), With<NodeMarker>>,
    other_query: Query<&Transform, With<NodeMarker>>,
    edge_query: Query<&Edge>,
) {
    for (mut acc, self_transform) in self_query.iter_mut() {
        // Apply gravity towards the center
        acc.value = -GRAVITY_CONSTANT * self_transform.translation;
        // info!(
        //     "Setting accelleration {:?} from translation {:?}",
        //     acc.value, self_transform.translation
        // );

        // Apply repulsive force between nodes
        for other_transform in other_query.iter() {
            let pos_diff = other_transform.translation - self_transform.translation;
            let len_sq = pos_diff.length_squared();
            if len_sq > 0.0 {
                acc.value -= pos_diff / pos_diff.length_squared() * REPULSION_CONSTANT;
            }
        }

        // info!("Finally set acceleration to {:?}", acc.value);
    }

    // Apply attraction force between connected nodes
    for edge in edge_query.iter() {
        for [self_id, other_id] in [[edge.left, edge.right], [edge.right, edge.left]] {
            let (mut self_acc, self_transform) = self_query
                .get_mut(self_id)
                .expect("Could not get edge node for spring force");

            let other_transform = other_query
                .get(other_id)
                .expect("Could not get edge node for spring force");

            let pos_diff = other_transform.translation - self_transform.translation;
            let dist_offset = pos_diff.length() - DESIRED_DISTANCE;
            self_acc.value += dist_offset * pos_diff.normalize() * SPRING_CONSTANT;
        }
    }
}

fn spring_update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds() / 1000.0;
    }
}

fn spring_update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let diff = velocity.value * time.delta_seconds();
        // info!("Moving node {diff}, from velocity {:?}", velocity.value);
        transform.translation += diff;
    }
}

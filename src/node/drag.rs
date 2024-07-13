use bevy::prelude::*;

use crate::camera::MainCamera;

use super::events::{NodeDragEndEvent, NodeDragEvent, NodeDragStartEvent};

#[derive(Component)]
pub struct Dragged;

pub struct DragPlugin;
impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drag_node.run_if(on_event::<NodeDragEvent>()))
            .add_systems(
                Update,
                drag_start_node.run_if(on_event::<NodeDragStartEvent>()),
            )
            .add_systems(Update, drag_end_node.run_if(on_event::<NodeDragEndEvent>()));
    }
}

fn drag_start_node(mut commands: Commands, mut drag_start_events: EventReader<NodeDragStartEvent>) {
    for drag_event in drag_start_events.read() {
        commands.entity(drag_event.node).insert(Dragged);
    }
}

fn drag_end_node(mut commands: Commands, mut drag_end_events: EventReader<NodeDragEndEvent>) {
    for drag_event in drag_end_events.read() {
        commands.entity(drag_event.node).remove::<Dragged>();
    }
}

fn drag_node(
    mut drag_events: EventReader<NodeDragEvent>,
    mut transform_query: Query<&mut Transform>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for drag_event in drag_events.read() {
        // Camera translation from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
        let (camera, camera_transform) = q_camera.single();

        info!("event delta {:?}", drag_event.delta);

        // This does not seem to be the correct translation...
        let delta: Vec3 = camera
            .viewport_to_world(camera_transform, drag_event.delta)
            .expect("Node should be within the camera world?")
            .origin
            - camera
                .viewport_to_world(camera_transform, Vec2::ZERO)
                .unwrap()
                .origin;

        info!("delta {:?}", delta);

        // let delta = drag_event.delta.extend(0.);

        let node_translation: &mut Vec3 = &mut transform_query
            .get_mut(drag_event.node)
            .expect("Did not find dragged node transform!")
            .translation;

        *node_translation += delta;
    }
}

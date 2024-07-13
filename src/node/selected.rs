use bevy::prelude::*;

use crate::node::{NodeClickEvent, NodeColor, NodeMarker};

#[derive(Component)]
pub struct Selected;

pub struct SelectedPlugin;
impl Plugin for SelectedPlugin {
    fn build(&self, app: &mut App) {
        // app.add_event::<MouseButtonInput>();
        app.add_systems(Update, selection_mouse_event.run_if(on_event::<NodeClickEvent>()))
            .add_systems(Update, deselection_mouse_event.run_if(on_event::<NodeClickEvent>()))
            .insert_resource(HighlightedNodeColor {
                color: Color::srgb(1.5, 0.75, 0.75),
            });
    }
}

#[derive(Resource, Clone, Debug)]
pub struct HighlightedNodeColor {
    pub color: Color,
}

// TODO: Improve this by just firing this method on the bubble-up click event
/// Select a clicked node
fn selection_mouse_event(
    mut commands: Commands,
    mut node_click_events: EventReader<NodeClickEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    color_query: Query<&Handle<ColorMaterial>, (With<NodeMarker>, Without<Selected>)>,
    highlight_color: Res<HighlightedNodeColor>,
) {
    for click_event in node_click_events.read() {
        if let Ok(color_handle) = color_query.get(click_event.node) {
            commands.entity(click_event.node).insert(Selected);
            materials
                .get_mut(color_handle)
                .expect("The color should be registered")
                .color = highlight_color.color.clone();
        };
    }
}

/// Deselects a node when we select another TODO: Deselect in other ways
fn deselection_mouse_event(
    mut commands: Commands,
    mut node_click_events: EventReader<NodeClickEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    color_query: Query<(&Handle<ColorMaterial>, Entity), (With<NodeMarker>, With<Selected>)>,
    node_color: Res<NodeColor>,
) {
    for click_event in node_click_events.read() {
        for (color_handle, entity) in color_query
            .iter()
            .filter(|(_, entity)| *entity != click_event.node)
        {
            commands.entity(entity).remove::<Selected>();
            materials
                .get_mut(color_handle)
                .expect("The color should be registered")
                .color = node_color.color.clone();
        }
    }
}

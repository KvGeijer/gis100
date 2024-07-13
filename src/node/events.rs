use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct NodeClickEvent {
    pub node: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for NodeClickEvent {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        Self { node: value.target }
    }
}

#[derive(Event)]
pub struct NodeDragEvent {
    pub node: Entity,
    pub delta: Vec2,
}

impl From<ListenerInput<Pointer<Drag>>> for NodeDragEvent {
    fn from(value: ListenerInput<Pointer<Drag>>) -> Self {
        Self {
            node: value.target,
            delta: value.event.delta,
        }
    }
}

#[derive(Event)]
pub struct NodeDragStartEvent {
    pub node: Entity,
}

impl From<ListenerInput<Pointer<DragStart>>> for NodeDragStartEvent {
    fn from(value: ListenerInput<Pointer<DragStart>>) -> Self {
        Self { node: value.target }
    }
}

#[derive(Event)]
pub struct NodeDragEndEvent {
    pub node: Entity,
}

impl From<ListenerInput<Pointer<DragEnd>>> for NodeDragEndEvent {
    fn from(value: ListenerInput<Pointer<DragEnd>>) -> Self {
        Self { node: value.target }
    }
}

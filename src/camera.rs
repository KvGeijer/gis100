use bevy::{prelude::*, render::camera::ScalingMode};
// use bevy_pancam::{PanCam, PanCamPlugin}; // Can add this back for panning. But might conflict with node selection?

// const CAMERA_DISTANCE: f32 = 40.0;

const CAMERA_Y_SIZE: f32 = 20.0;

#[derive(Component, Debug)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            //     .add_plugins(PanCamPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // The sizes are adapted so that a node radius is the unitary unit
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(CAMERA_Y_SIZE);

    commands.spawn((
        camera, MainCamera,
        // PanCam::default()
    ));

    // TODO: Understand why this does not work, and how to fix it
    // Would be cool to have a 3d camera with poster mode for 2d things, being able to rotate the view
    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE)
    //             .looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     },
    //     MainCamera,
    // ));
}

use bevy::app::App;
use bevy::input::mouse::MouseWheel;
use bevy::input::Input;
use bevy::prelude::{
    Camera, EventReader, KeyCode, Plugin, Projection, Query, Res, Transform, Update, With,
};

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_camera, zoom_camera));
    }
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut();

    // Adjust camera_system position based on keyboard input
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.z -= 0.1;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.z += 0.1;
    }
    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= 0.1;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += 0.1;
    }
}

fn zoom_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera>>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        if let Ok(mut projection) = query.get_single_mut() {
            if let Projection::Orthographic(ref mut orthographic) = *projection {
                orthographic.scale -= mouse_wheel.y * 0.1;
                orthographic.scale = orthographic.scale.clamp(0.5, 10.);
            }
        }
    }
}

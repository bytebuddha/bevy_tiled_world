use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn move_camera(
    keys: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>
) {
    for mut transform in camera.iter_mut() {
        if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
            transform.translation.y += 30.0;
        }
        if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
            transform.translation.y -= 30.0;
        }
        if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
            transform.translation.x -= 30.0;
        }
        if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
            transform.translation.x += 30.0;
        }

    }
}

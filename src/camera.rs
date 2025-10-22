use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseMotion};
use crate::config::*;

/// Component to mark the orbital camera
#[derive(Component)]
pub struct OrbitalCamera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,   // Rotation around Y axis (horizontal)
    pub pitch: f32, // Rotation around X axis (vertical)
}

impl Default for OrbitalCamera {
    fn default() -> Self {
        Self {
            target: Vec3::new(
                (WORLD_WIDTH as f32 * VOXEL_SIZE) / 2.0,
                (WORLD_HEIGHT as f32 * VOXEL_SIZE) / 4.0,
                (WORLD_DEPTH as f32 * VOXEL_SIZE) / 2.0,
            ),
            distance: CAMERA_INITIAL_DISTANCE,
            yaw: 45.0_f32.to_radians(),
            pitch: 30.0_f32.to_radians(),
        }
    }
}

/// Setup the camera
pub fn setup_camera(mut commands: Commands) {
    let camera = OrbitalCamera::default();
    let position = calculate_camera_position(&camera);

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(position).looking_at(camera.target, Vec3::Y),
        camera,
    ));
}

/// Calculate camera position from orbital parameters
fn calculate_camera_position(camera: &OrbitalCamera) -> Vec3 {
    let x = camera.distance * camera.pitch.cos() * camera.yaw.sin();
    let y = camera.distance * camera.pitch.sin();
    let z = camera.distance * camera.pitch.cos() * camera.yaw.cos();

    camera.target + Vec3::new(x, y, z)
}

/// System to handle camera rotation with mouse
pub fn camera_rotation_system(
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut OrbitalCamera, &mut Transform)>,
) {
    if !mouse_button.pressed(MouseButton::Right) {
        return;
    }

    let delta: Vec2 = mouse_motion.read().map(|m| m.delta).sum();

    for (mut camera, mut transform) in camera_query.iter_mut() {
        camera.yaw += delta.x * 0.005 * CAMERA_ROTATE_SPEED;
        camera.pitch -= delta.y * 0.005 * CAMERA_ROTATE_SPEED;

        // Clamp pitch to avoid gimbal lock
        camera.pitch = camera.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        let position = calculate_camera_position(&camera);
        transform.translation = position;
        transform.look_at(camera.target, Vec3::Y);
    }
}

/// System to handle camera zoom with mouse wheel
pub fn camera_zoom_system(
    mut scroll: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut OrbitalCamera, &mut Transform)>,
) {
    let delta: f32 = scroll.read().map(|e| e.y).sum();

    if delta.abs() < 0.001 {
        return;
    }

    for (mut camera, mut transform) in camera_query.iter_mut() {
        camera.distance -= delta * CAMERA_ZOOM_SPEED;
        camera.distance = camera.distance.clamp(10.0, 500.0);

        let position = calculate_camera_position(&camera);
        transform.translation = position;
        transform.look_at(camera.target, Vec3::Y);
    }
}

/// System to handle camera panning with keyboard
pub fn camera_pan_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut OrbitalCamera, &mut Transform)>,
) {
    let mut pan = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        pan.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        pan.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        pan.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        pan.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Space) {
        pan.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        pan.y -= 1.0;
    }

    if pan.length() > 0.0 {
        pan = pan.normalize() * CAMERA_MOVE_SPEED * time.delta_secs();

        for (mut camera, mut transform) in camera_query.iter_mut() {
            camera.target += pan;

            let position = calculate_camera_position(&camera);
            transform.translation = position;
            transform.look_at(camera.target, Vec3::Y);
        }
    }
}

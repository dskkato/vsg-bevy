use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    window::{CursorMoved, PresentMode},
};

mod stim;
use stim::{CustomMaterial, CustomMaterialPlugin};

fn main() {
    let window_size = WindowSize(Vec2::new(800.0, 800.0));
    App::new()
        .insert_resource(WindowDescriptor {
            width: window_size.0.x,
            height: window_size.0.y,
            title: "VSG".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(window_size)
        .add_plugins(DefaultPlugins)
        .add_plugin(CustomMaterialPlugin)
        .add_startup_system(setup)
        .add_system(print_mouse_events_system)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let angle = StimAngle(0.0);
    let position = StimPosition(0.0, 0.0);
    // cube
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Circle::new(0.5))),
        Transform::from_xyz(position.0, position.1, -1.0)
            .with_rotation(Quat::from_rotation_z(angle.0)),
        GlobalTransform::default(),
        CustomMaterial,
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_3d());
    commands.insert_resource(angle);
    commands.insert_resource(position);
}

struct StimAngle(f32);
struct StimPosition(f32, f32);
struct WindowSize(Vec2);

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<CustomMaterial>>,
    mut angle: ResMut<StimAngle>,
    mut position: ResMut<StimPosition>,
    window_size: Res<WindowSize>,
) {
    for event in cursor_moved_events.iter() {
        let x = 2.0 * event.position.x / window_size.0.x - 1.0;
        let y = 2.0 * event.position.y / window_size.0.y - 1.0;
        *position = StimPosition(x, y);
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        angle.0 += 0.1 * event.y;
        info!("{:?}", angle.0);
    }

    for mut transform in query.iter_mut() {
        *transform = Transform::from_xyz(position.0, position.1, -1.0)
            .with_rotation(Quat::from_rotation_z(angle.0));
    }
}

use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    window::{CursorMoved, PresentMode},
};
use bevy_egui::{egui, EguiContext, EguiPlugin};

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
            resizable: true,
            ..default()
        })
        .insert_resource(window_size)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(CustomMaterialPlugin)
        .add_startup_system(setup)
        .add_system(print_mouse_events_system)
        .add_system(ui_example)
        .run();
}

#[derive(Component, Debug)]
enum StimType {
    Grating,
    Gabor,
}

fn ui_example(
    mut ctx: ResMut<EguiContext>,
    mut stim_type: ResMut<StimType>,
    mut commands: Commands,
    query: Query<Entity, With<Grating>>,
) {
    egui::Window::new("Hello").show(ctx.ctx_mut(), |ui| {
        ui.heading("Stim type");
        ui.label("world");
        if ui.button("gabor").clicked() {
            match *stim_type {
                StimType::Gabor => *stim_type = StimType::Grating,
                StimType::Grating => *stim_type = StimType::Gabor,
            }
            info!("clicked {:?}", stim_type);
            if let Some(entity) = query.iter().next() {
                info!("query");
                commands.entity(entity).remove_bundle::<MyComponent>();
            }
        }
    });
}

#[derive(Component, Bundle)]
struct MyComponent {
    mesh: Handle<Mesh>,
    transform: Transform,
    global_transform: GlobalTransform,
    material: CustomMaterial,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let param = StimParam {
        angle: 0.0,
        pos: Vec2::new(0.0, 0.0),
    };
    // cube
    commands
        .spawn_bundle(MyComponent {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.5, 0.5)))),
            transform: Transform::from_xyz(param.pos.x, param.pos.y, -1.0)
                .with_rotation(Quat::from_rotation_z(param.angle)),
            global_transform: GlobalTransform::default(),
            material: CustomMaterial,
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        })
        .insert(Grating);

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_3d());
    commands.insert_resource(param);
    commands.insert_resource(StimType::Grating);
}

#[derive(Component)]
struct Grating;

struct StimParam {
    pub angle: f32,
    pub pos: Vec2,
}
struct WindowSize(Vec2);

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<CustomMaterial>>,
    mut param: ResMut<StimParam>,
    window_size: Res<WindowSize>,
) {
    for event in cursor_moved_events.iter() {
        let x = 2.0 * event.position.x / window_size.0.x - 1.0;
        let y = 2.0 * event.position.y / window_size.0.y - 1.0;
        param.pos.x = x;
        param.pos.y = y;
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        param.angle += 0.1 * event.y;
        info!("{:?}", param.angle);
    }

    for mut transform in query.iter_mut() {
        *transform = Transform::from_xyz(param.pos.x, param.pos.y, -1.0)
            .with_rotation(Quat::from_rotation_z(param.angle));
    }
}

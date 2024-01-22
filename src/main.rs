use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct HelloGnoemPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloGnoemPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_camera, zoom_camera))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(32.0).into()),
        material: materials.add(Color::WHITE.into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(3.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut();

    // Adjust camera position based on keyboard input
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

impl Plugin for HelloGnoemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_gnoems)
            .add_systems(Update, greet_gnoems);
    }
}

fn add_gnoems(mut commands: Commands) {
    commands.spawn((Gnoem, Name("Willy".to_string())));
    commands.spawn((Gnoem, Name("Kaulana".to_string())));
    commands.spawn((Gnoem, Name("Buhle".to_string())));
}

fn greet_gnoems(query: Query<&Name, With<Gnoem>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

#[derive(Component)]
struct Gnoem;

#[derive(Component)]
struct Name(String);

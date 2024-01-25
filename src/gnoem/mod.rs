use crate::gnoem;
use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::RigidBody;
use rand::Rng;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Velocity(f32);

#[derive(Bundle)]
struct GnoemBundle {
    name: Name,
    rigid_body: RigidBody,
    pbr: PbrBundle,
    collider: Collider,
    velocity: Velocity,
}

pub(crate) struct GnoemPlugin;

impl Plugin for GnoemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveTimer(Timer::from_seconds(
            (1. / 1000.) * 50.,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, spawn_gnoem)
        .add_systems(Update, move_gnoem);
    }
}

fn spawn_gnoem(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    command.spawn(GnoemBundle {
        name: Name("Gnoem".into()),
        rigid_body: RigidBody::Dynamic,
        pbr: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 2.5, 1.))),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0., 5., 0.),
            ..default()
        },
        collider: Collider::cuboid(0.5, 1.25, 0.5),
        velocity: Velocity(0.),
    });
}

#[derive(Resource)]
struct MoveTimer(Timer);

fn move_gnoem(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut query: Query<&mut Transform, With<Name>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut transform in query.iter_mut() {
            // Define the center of the radius (e.g., (0.0, 0.0))
            let center = Vec3::new(0.0, 0.0, 0.0);
            let radius = 10.0; // Adjust this radius as needed
            let speed = 10.0; // Adjust the speed as needed

            // Calculate the distance from current position to the center
            let distance = transform.translation.distance(center);

            let direction = if distance > radius {
                // Move towards the center if outside the radius
                (center - transform.translation).normalize()
            } else {
                // Move in a random direction if inside the radius
                let mut rng = rand::thread_rng();
                let random_angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                Vec3::new(random_angle.cos(), random_angle.sin(), 0.0).normalize()
            };

            // Update the entity's position based on its velocity and elapsed time
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}

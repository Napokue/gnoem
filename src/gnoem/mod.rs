use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::RigidBody;

#[derive(Component)]
struct Name(String);

#[derive(Bundle)]
struct GnoemBundle {
    name: Name,
    rigid_body: RigidBody,
    pbr: PbrBundle,
    collider: Collider,
}

pub(crate) struct GnoemPlugin;

impl Plugin for GnoemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gnoem);
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
    });
}

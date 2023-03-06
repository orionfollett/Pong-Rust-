use bevy::{
    prelude::{
        default, shape, App, Assets, Color, Commands, Component, Mesh, Plugin, ResMut, Transform,
        Vec3,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, Friction, LockedAxes, RigidBody};

const PADDLE_WIDTH: f32 = 20.;
const PADDLE_HEIGHT: f32 = 80.;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paddle);
    }
}

#[derive(Component)]
pub struct Paddle;

fn setup_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HEIGHT / 2.))
        .insert(Friction::coefficient(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(-200.0, 0.0, 0.0)))
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Paddle);
}

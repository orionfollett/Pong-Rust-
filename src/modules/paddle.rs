use bevy::{
    prelude::{
        default, shape, App, Assets, Color, Commands, Component, Mesh, Plugin, Query, ResMut,
        Transform, Vec2, Vec3, With, Without,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, Friction, LockedAxes, Restitution, RigidBody, Velocity};

use super::ball::Ball;

const PADDLE_WIDTH: f32 = 20.;
const PADDLE_HEIGHT: f32 = 80.;

pub const PADDLE_SPEED: f32 = 500.;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paddle)
            .add_system(move_ai_paddle);
    }
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct ComputerControlledPaddle;

fn setup_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HEIGHT / 2.))
        .insert(Friction::coefficient(0.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.,
        })
        .insert(Restitution::coefficient(1.0))
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(-200., 0.0, 0.0)))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Paddle);

    commands
        .spawn(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HEIGHT / 2.))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.,
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(200., 0.0, 0.0)))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ComputerControlledPaddle);
}

fn move_ai_paddle(
    mut ai_paddles: Query<&mut Transform, (With<ComputerControlledPaddle>, Without<Ball>)>,
    balls: Query<&Transform, (With<Ball>, Without<ComputerControlledPaddle>)>,
) {
    //just follows first ball
    match balls.get_single() {
        Ok(ball) => {
            for mut paddle in ai_paddles.iter_mut() {
                paddle.translation.y = ball.translation.y
            }
        }
        Err(_) => todo!(),
    }
}

use bevy::{
    prelude::{
        default, shape, App, Assets, Color, Commands, Component, Mesh, Plugin, ResMut, Transform,
        Vec2, Vec3,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{
    Collider, ExternalImpulse, Friction, GravityScale, LockedAxes, Restitution, RigidBody, Sensor,
    Velocity,
};

pub const BALL_RADIUS: f32 = 20.0;
pub const BALL_SPAWN_VELOCITY: Vec2 = Vec2::new(500.0, 200.0);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ball)
            .add_startup_system(setup_boundaries);
        //.add_system(print_ball_position);
    }
}

#[derive(Component)]
pub struct Ball;

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(BALL_RADIUS))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
            ..default()
        })
        .insert(GravityScale(0.0))
        .insert(Velocity {
            linvel: BALL_SPAWN_VELOCITY,
            angvel: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Ball);
}

fn setup_boundaries(mut commands: Commands) {
    //floor
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -275.0, 0.0)));

    //ceiling
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 275.0, 0.0)));

    //left wall
    commands
        .spawn(Collider::cuboid(50.0, 500.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(-275.0, 0.0, 0.0)))
        .insert(Sensor);

    //right wall
    commands
        .spawn(Collider::cuboid(50.0, 500.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(275.0, 0.0, 0.0)));
}

// fn print_ball_position(balls : Query<&Transform, With<Ball>>) {
//     for transform in balls.iter(){
//         println!("{:?}", transform.translation);
//     }
// }

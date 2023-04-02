use super::{
    ball::Ball, ball::BALL_SPAWN_VELOCITY, display::SCREEN_WIDTH, paddle::ComputerControlledPaddle,
};
use bevy::prelude::{App, Plugin, Query, Transform, Vec2, With, Without};
use bevy_rapier2d::prelude::{RigidBody, Velocity};
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(score_detection);
    }
}

//updates the state of the game

//events:
//ball goes off screen: update player score, and restart ball in the center

//that's it..

fn score_detection(
    mut balls_transform: Query<&mut Transform, (With<Ball>, Without<ComputerControlledPaddle>)>,
    mut balls_vel: Query<&mut Velocity, (With<Ball>, Without<ComputerControlledPaddle>)>,
) {
    for mut ball in balls_transform.iter_mut() {
        if ball.translation.x < (SCREEN_WIDTH / -2.0) {
            for mut vel in balls_vel.iter_mut() {
                vel.linvel = BALL_SPAWN_VELOCITY;
            }
            ball.translation.x = 0.0;
            ball.translation.y = 0.0;
        }
    }
}

use bevy::prelude::{App, Input, KeyCode, Plugin, Query, Res, Transform, Vec2, With};
use bevy_rapier2d::prelude::Velocity;

use super::paddle::Paddle;
use super::paddle::{self, PADDLE_SPEED};

pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_input);
    }
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut paddles: Query<&mut Velocity, With<Paddle>>,
) {
    for mut paddle in paddles.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            paddle.linvel = Vec2::new(0., PADDLE_SPEED);
        } else if keyboard_input.pressed(KeyCode::Down) {
            paddle.linvel = Vec2::new(0., -1. * PADDLE_SPEED);
        } else {
            paddle.linvel = Vec2::new(0., 0.);
        }
    }
}

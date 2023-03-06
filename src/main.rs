mod modules;
use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use modules::{
    ball::BallPlugin,
    display::{DisplayPlugin, SCREEN_HEIGHT, SCREEN_WIDTH},
    paddle::PaddlePlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugin(DisplayPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(PaddlePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

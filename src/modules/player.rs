use bevy::{
    prelude::{default, App, AssetServer, Commands, Component, Plugin, Res, Transform, Query, With},
    sprite::SpriteBundle,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player);
    }
}

#[derive(Component)]
struct Player;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("../assets/box.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
    ));
}

fn input_system(time: Res<Time>, mut player_position: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut player_position{

    }
}



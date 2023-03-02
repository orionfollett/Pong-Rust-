use bevy::{
    prelude::{App, AssetServer, Camera2dBundle, Commands, Component, Plugin, Res, ResMut},
    window::Windows,
};

//Display Plugin
pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_window)
            .add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

//system to set window title
fn setup_window(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title(String::from("Rust Pong"));
    //window.set_decorations(false);
}

#[derive(Component)]
struct Display;

fn display_sprites(mut commands: Commands) {}

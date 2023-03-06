use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{App, Camera2d, Camera2dBundle, Color, Commands, Plugin, ResMut},
    window::Windows,
};

//Display Plugin

pub const SCREEN_WIDTH: f32 = 500.0;
pub const SCREEN_HEIGHT: f32 = 500.0;

pub struct DisplayPlugin;
impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_window)
            .add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0., 0., 0.)),
        },
        ..Default::default()
    });
}

fn setup_window(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title(String::from("Rust Pong"));
    //window.set_decorations(false);
}

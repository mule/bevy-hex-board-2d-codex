use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_hex_board_2d_codex::{APP_TITLE, default_window_resolution};

fn main() {
    let (window_width, window_height) = default_window_resolution();

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_TITLE.into(),
                resolution: WindowResolution::new(window_width, window_height),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

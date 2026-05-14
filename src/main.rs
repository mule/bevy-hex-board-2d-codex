use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_hex_board_2d_codex::{
    APP_TITLE,
    board::DEFAULT_BOARD_RADIUS,
    board_render::{BoardPlugin, BoardSettings},
    default_window_resolution,
};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(
        long,
        allow_hyphen_values = true,
        default_value_t = DEFAULT_BOARD_RADIUS,
        value_parser = parse_board_radius,
        help = "Radius of the generated hex board"
    )]
    radius: i32,
}

fn main() {
    let cli = Cli::parse();
    let (window_width, window_height) = default_window_resolution();

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .insert_resource(BoardSettings {
            radius: cli.radius,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_TITLE.into(),
                resolution: WindowResolution::new(window_width, window_height),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BoardPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn parse_board_radius(value: &str) -> Result<i32, String> {
    let radius = value
        .parse::<i32>()
        .map_err(|error| format!("radius must be an integer: {error}"))?;

    if radius < 0 {
        return Err("radius must be non-negative".to_string());
    }

    Ok(radius)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_uses_default_board_radius() {
        let cli = Cli::parse_from(["bevy-hex-board-2d-codex"]);

        assert_eq!(cli.radius, DEFAULT_BOARD_RADIUS);
    }

    #[test]
    fn cli_accepts_radius_argument() {
        let cli = Cli::parse_from(["bevy-hex-board-2d-codex", "--radius", "6"]);

        assert_eq!(cli.radius, 6);
    }

    #[test]
    fn cli_rejects_negative_radius() {
        let error = Cli::try_parse_from(["bevy-hex-board-2d-codex", "--radius", "-1"])
            .expect_err("negative radius should be rejected");

        assert_eq!(error.kind(), clap::error::ErrorKind::ValueValidation);
    }
}

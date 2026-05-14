use bevy::prelude::*;

use crate::board::{DEFAULT_BOARD_RADIUS, axial_to_world, generate_hex_board};

const HEX_SIDES: u32 = 6;
const DEFAULT_TILE_SIZE: f32 = 40.0;
const DEFAULT_OUTLINE_THICKNESS: f32 = 3.0;
const TILE_Z: f32 = 0.0;
const OUTLINE_Z: f32 = 0.1;

#[derive(Default)]
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardSettings>()
            .add_systems(Startup, spawn_board);
    }
}

#[derive(Clone, Copy, Debug, Resource)]
pub struct BoardSettings {
    pub radius: i32,
    pub tile_size: f32,
    pub outline_thickness: f32,
    pub tile_color: Color,
    pub outline_color: Color,
}

impl Default for BoardSettings {
    fn default() -> Self {
        Self {
            radius: DEFAULT_BOARD_RADIUS,
            tile_size: DEFAULT_TILE_SIZE,
            outline_thickness: DEFAULT_OUTLINE_THICKNESS,
            tile_color: Color::srgb(0.22, 0.42, 0.50),
            outline_color: Color::srgb(0.06, 0.08, 0.10),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct HexTile;

#[derive(Component, Clone, Copy, Debug, Default)]
struct HexTileOutline;

fn spawn_board(
    mut commands: Commands,
    settings: Res<BoardSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile_mesh = meshes.add(RegularPolygon::new(settings.tile_size, HEX_SIDES));
    let outline_mesh = meshes.add(
        RegularPolygon::new(settings.tile_size, HEX_SIDES).to_ring(settings.outline_thickness),
    );
    let tile_material = materials.add(settings.tile_color);
    let outline_material = materials.add(settings.outline_color);

    for coord in generate_hex_board(settings.radius) {
        let position = axial_to_world(coord, settings.tile_size);

        commands
            .spawn((
                HexTile,
                coord,
                Mesh2d(tile_mesh.clone()),
                MeshMaterial2d(tile_material.clone()),
                Transform::from_xyz(position.x, position.y, TILE_Z),
            ))
            .with_children(|tile| {
                tile.spawn((
                    HexTileOutline,
                    Mesh2d(outline_mesh.clone()),
                    MeshMaterial2d(outline_material.clone()),
                    Transform::from_xyz(0.0, 0.0, OUTLINE_Z),
                ));
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::hex_board_tile_count;

    #[test]
    fn default_board_settings_are_valid() {
        let settings = BoardSettings::default();

        assert_eq!(settings.radius, DEFAULT_BOARD_RADIUS);
        assert!(settings.radius >= 0);
        assert!(settings.tile_size > 0.0);
        assert!(settings.outline_thickness > 0.0);
        assert!(settings.outline_thickness < settings.tile_size);
    }

    #[test]
    fn board_plugin_spawns_one_tile_per_generated_coordinate() {
        let mut app = App::new();

        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<ColorMaterial>>()
            .add_plugins(BoardPlugin);

        app.update();

        let world = app.world_mut();
        let mut query = world.query::<&HexTile>();
        let tile_count = query.iter(world).count();

        assert_eq!(tile_count, hex_board_tile_count(DEFAULT_BOARD_RADIUS));
    }
}

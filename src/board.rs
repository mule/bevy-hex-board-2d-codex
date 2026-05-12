pub const DEFAULT_BOARD_RADIUS: i32 = 4;
const SQRT_3: f32 = 1.732_050_8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

impl HexCoord {
    pub const ZERO: Self = Self { q: 0, r: 0 };

    pub const fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub const fn s(self) -> i32 {
        -self.q - self.r
    }

    pub fn neighbor(self, direction: HexDirection) -> Self {
        let offset = direction.offset();

        Self::new(self.q + offset.q, self.r + offset.r)
    }

    pub fn neighbors(self) -> [Self; 6] {
        HexDirection::ALL.map(|direction| self.neighbor(direction))
    }

    pub fn distance_to(self, other: Self) -> i32 {
        let dq = (self.q - other.q).abs();
        let dr = (self.r - other.r).abs();
        let ds = (self.s() - other.s()).abs();

        dq.max(dr).max(ds)
    }

    pub fn is_within_radius(self, radius: i32) -> bool {
        self.distance_to(Self::ZERO) <= radius
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HexDirection {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

impl HexDirection {
    pub const ALL: [Self; 6] = [
        Self::East,
        Self::NorthEast,
        Self::NorthWest,
        Self::West,
        Self::SouthWest,
        Self::SouthEast,
    ];

    pub const fn offset(self) -> HexCoord {
        match self {
            Self::East => HexCoord::new(1, 0),
            Self::NorthEast => HexCoord::new(1, -1),
            Self::NorthWest => HexCoord::new(0, -1),
            Self::West => HexCoord::new(-1, 0),
            Self::SouthWest => HexCoord::new(-1, 1),
            Self::SouthEast => HexCoord::new(0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
}

impl WorldPosition {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn generate_hex_board(radius: i32) -> Vec<HexCoord> {
    assert!(radius >= 0, "board radius must be non-negative");

    let mut coords = Vec::with_capacity(hex_board_tile_count(radius));

    for q in -radius..=radius {
        let r_min = (-radius).max(-q - radius);
        let r_max = radius.min(-q + radius);

        for r in r_min..=r_max {
            coords.push(HexCoord::new(q, r));
        }
    }

    coords
}

pub fn hex_board_tile_count(radius: i32) -> usize {
    assert!(radius >= 0, "board radius must be non-negative");

    let radius = radius as usize;

    1 + 3 * radius * (radius + 1)
}

pub fn axial_to_world(coord: HexCoord, hex_size: f32) -> WorldPosition {
    assert!(hex_size > 0.0, "hex size must be positive");

    let q = coord.q as f32;
    let r = coord.r as f32;

    WorldPosition::new(SQRT_3 * hex_size * (q + r / 2.0), 1.5 * hex_size * r)
}

pub fn world_to_axial(position: WorldPosition, hex_size: f32) -> HexCoord {
    assert!(hex_size > 0.0, "hex size must be positive");

    let q = (SQRT_3 / 3.0 * position.x - position.y / 3.0) / hex_size;
    let r = (2.0 / 3.0 * position.y) / hex_size;

    round_axial(q, r)
}

fn round_axial(q: f32, r: f32) -> HexCoord {
    let s = -q - r;

    let mut q_round = q.round();
    let mut r_round = r.round();
    let s_round = s.round();

    let q_diff = (q_round - q).abs();
    let r_diff = (r_round - r).abs();
    let s_diff = (s_round - s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        q_round = -r_round - s_round;
    } else if r_diff > s_diff {
        r_round = -q_round - s_round;
    }

    HexCoord::new(q_round as i32, r_round as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEX_SIZE: f32 = 32.0;

    #[test]
    fn hex_neighbors_include_six_tiles() {
        let neighbors = HexCoord::ZERO.neighbors();

        assert_eq!(neighbors.len(), 6);
        assert!(neighbors.contains(&HexCoord::new(1, 0)));
        assert!(neighbors.contains(&HexCoord::new(1, -1)));
        assert!(neighbors.contains(&HexCoord::new(0, -1)));
        assert!(neighbors.contains(&HexCoord::new(-1, 0)));
        assert!(neighbors.contains(&HexCoord::new(-1, 1)));
        assert!(neighbors.contains(&HexCoord::new(0, 1)));
    }

    #[test]
    fn radius_zero_board_has_one_tile() {
        let board = generate_hex_board(0);

        assert_eq!(board, vec![HexCoord::ZERO]);
    }

    #[test]
    fn radius_one_board_has_seven_tiles() {
        let board = generate_hex_board(1);

        assert_eq!(board.len(), 7);
        assert_eq!(board.len(), hex_board_tile_count(1));
    }

    #[test]
    fn generated_board_contains_only_in_radius_coordinates() {
        for radius in 0..=DEFAULT_BOARD_RADIUS {
            let board = generate_hex_board(radius);

            assert_eq!(board.len(), hex_board_tile_count(radius));
            assert!(board.iter().all(|coord| coord.is_within_radius(radius)));
        }
    }

    #[test]
    fn hex_distance_behaves_for_representative_coordinates() {
        assert_eq!(HexCoord::ZERO.distance_to(HexCoord::ZERO), 0);
        assert_eq!(HexCoord::ZERO.distance_to(HexCoord::new(1, 0)), 1);
        assert_eq!(HexCoord::ZERO.distance_to(HexCoord::new(2, -1)), 2);
        assert_eq!(HexCoord::new(-2, 1).distance_to(HexCoord::new(1, -2)), 3);
    }

    #[test]
    fn axial_world_conversion_round_trips_for_tile_centers() {
        for coord in generate_hex_board(DEFAULT_BOARD_RADIUS) {
            let world_position = axial_to_world(coord, HEX_SIZE);
            let converted_coord = world_to_axial(world_position, HEX_SIZE);

            assert_eq!(converted_coord, coord);
        }
    }
}

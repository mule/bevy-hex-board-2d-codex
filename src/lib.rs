pub mod board;

pub const APP_TITLE: &str = "Bevy Hex Board 2D";
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub fn default_window_resolution() -> (u32, u32) {
    (WINDOW_WIDTH, WINDOW_HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_title_is_not_empty() {
        assert!(!APP_TITLE.is_empty());
    }

    #[test]
    fn default_window_size_is_positive() {
        let (width, height) = default_window_resolution();

        assert!(width > 0);
        assert!(height > 0);
    }
}

use rocket::http::uri::Absolute;

pub mod friend;
pub mod job;
pub mod network;
pub mod project;
pub mod talk;

pub use self::friend::*;
pub use self::job::*;
pub use self::network::*;
pub use self::project::*;
pub use self::talk::*;
pub use crate::wallpapers::WALLPAPERS;

pub struct Logo {
    pub file: &'static str,
    pub transparent_background: bool,
}

pub struct Link {
    pub label: &'static str,
    pub uri: Absolute<'static>,
}

pub struct Location {
    pub precise: &'static str,
    pub broad: &'static str,
    pub latitude: f32,
    pub longitude: f32,
}
pub struct Wallpaper {
    pub file: &'static str,
    pub date: &'static str,
    pub location: Location,
}

impl Wallpaper {
    pub fn random() -> Option<&'static &'static Wallpaper> {
        use nanorand::{ChaCha20, Rng};
        use std::ops::Range;

        let range = Range {
            start: 0,
            end: WALLPAPERS.len(),
        };

        WALLPAPERS.get(ChaCha20::new().generate_range(range))
    }
}

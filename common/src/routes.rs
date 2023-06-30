//! Defines routes for apis.

use std::fmt;

/// The server's tcp port for clients to connect to during development.
pub const DEV_PORT: u16 = 2506;

/// The name of the CSS file.
pub const CSS_FILE_NAME: &str = "primer.css";

/// The path segment for static assets.
pub const ASSETS_PATH: &str = "assets";

/// The various routes of the app.
pub enum PageRoutes {
    Home,
    Signup,
}

impl fmt::Display for PageRoutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Home => write!(f, "/"),
            Self::Signup => write!(f, "signup"),
        }
    }
}

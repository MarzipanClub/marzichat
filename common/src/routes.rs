//! Defines routes for apis.

use {const_format::formatcp as f, std::fmt};

/// The server's tcp port for clients to connect to during development.
pub const DEV_PORT: u16 = 2506;

/// The site's domain name.
pub const WEBSITE_DOMAIN: &str = "marzichat.com";

/// The name of the CSS file.
pub const CSS_FILE_NAME: &str = "primer.css";

/// The path segment for static assets.
pub const ASSETS_PATH: &str = "assets";

/// The path segment for all programmatic routes
/// that are not supposed to be accessed by the end users.
const API: &str = "api";

/// The route for the backend health checks.
pub const HEALTH: &str = f!("/{API}/health");

/// The route to the compile-time info.
pub const INFO: &str = f!("/{API}/info");

/// The route to the websocket api.
pub const WEBSOCKET: &str = f!("/{API}/ws");

#[cfg(debug_assertions)]
pub const WEBSOCKET_URL: &str = f!("ws://localhost:{DEV_PORT}{WEBSOCKET}");

#[cfg(not(debug_assertions))]
pub const WEBSOCKET_URL: &str = f!("wss://{WEBSITE_DOMAIN}{WEBSOCKET}");

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

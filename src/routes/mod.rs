use {leptos_router::ToHref, std::fmt};

pub mod not_found;
pub mod signin;
pub mod signup;
pub mod stories;
pub mod story;
pub mod users;

const ROOT: &str = "/";
const ABOUT: &str = "/about";
const NEWSLETTER: &str = "/newsletter";
const SIGNIN: &str = "/signin";
const SIGNUP: &str = "/signup";

/// The route to a page in the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Routes {
    Home,
    About,
    Newsletter,
    Signin,
    Signup,
}

impl fmt::Display for Routes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Routes::Home => write!(f, "{ROOT}"),
            Routes::About => write!(f, "{ABOUT}"),
            Routes::Newsletter => write!(f, "{NEWSLETTER}"),
            Routes::Signin => write!(f, "{SIGNIN}"),
            Routes::Signup => write!(f, "{SIGNUP}"),
        }
    }
}

impl From<Routes> for String {
    fn from(route: Routes) -> Self {
        route.to_string()
    }
}

impl ToHref for Routes {
    fn to_href(&self) -> Box<dyn Fn() -> String> {
        match self {
            Routes::Home => Box::new(|| ROOT.into()),
            Routes::About => Box::new(|| ABOUT.into()),
            Routes::Newsletter => Box::new(|| NEWSLETTER.into()),
            Routes::Signin => Box::new(|| SIGNIN.into()),
            Routes::Signup => Box::new(|| SIGNUP.into()),
        }
    }
}

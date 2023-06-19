//! Translations module.
//!
//! This module contains all the translations for the copy used in the
//! app.

use crate::internationalization::Language;

/// Contains all translations for a given [Language][Language].
///
/// Usage:
/// ```
/// # use crate::internationalization::{self, Language};
/// let t = internationalization::Translations::for_language(Language::English);
/// let your_posts = t.your_posts();
/// println!("{your_posts}");
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Translations(Language);

macro_rules! translations {
    ($name:ident, $en:expr, $es:expr, $de:expr, $fr:expr) => {
        pub fn $name(self) -> &'static str {
            match self.0 {
                Language::English => $en,
                Language::Spanish => $es,
                Language::German => $de,
                Language::French => $fr,
            }
        }
    };
}

impl Translations {
    translations!(
        signup,
        "Signup",
        "Inscribirse",
        "Registrieren",
        "S'inscrire"
    );

    /// Create a new `Translations` instance for the given `Language`.
    pub fn for_language(language: Language) -> Self {
        Self(language)
    }
}

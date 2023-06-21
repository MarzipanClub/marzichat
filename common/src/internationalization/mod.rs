//! Internationalization module.

mod language;
mod translations;

pub use {
    language::{Language, LANGUAGES},
    translations::Translations,
};

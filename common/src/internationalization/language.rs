//! Language
//!
//! This module contains the `Language` enum and related helper methods.

use {
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
    std::fmt::Display,
};

/// The various langauges that the application supports.
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, From, Hash)]
pub enum Language {
    /// American English
    English,

    /// español mexicano con "tú"
    Spanish,

    /// Standardhochdeutsch mit „du“
    #[default]
    German,

    /// le français standard avec «tu»
    French,
}

/// A list of all supported languages.
pub const LANGUAGES: &[Language; 4] = &[
    Language::English,
    Language::Spanish,
    Language::German,
    Language::French,
];

const EN_US: &str = "en-US";
const ES_MX: &str = "es-MX";
const DE_DE: &str = "de-DE";
const FR_FR: &str = "fr-FR";

/// An error that occurs when parsing a language from a BCP 47 tag.
#[derive(thiserror::Error, Debug, Display, PartialEq, Eq, Clone, Copy)]
pub struct ParseError;

impl Language {
    /// As IETF BCP 47 language tag.
    pub const fn as_bcp47_tag(self) -> &'static str {
        match self {
            Language::English => EN_US,
            Language::Spanish => ES_MX,
            Language::German => DE_DE,
            Language::French => FR_FR,
        }
    }

    /// From IETF BCP 47 language tag.
    pub fn parse_from_bcp47_tag(tag: &str) -> Result<Language, ParseError> {
        match tag {
            EN_US => Ok(Language::English),
            ES_MX => Ok(Language::Spanish),
            DE_DE => Ok(Language::German),
            FR_FR => Ok(Language::French),
            _ => Err(ParseError),
        }
    }

    /// Convert the language to Open Graph locale.
    pub fn to_open_graph_locale(self) -> String {
        // open graph specifies the format to be language_TERRITORY
        self.as_bcp47_tag().replace('-', "_")
    }

    /// Alternate Open Graph locales.
    pub fn to_alternate_open_graph_locales(self) -> Vec<String> {
        LANGUAGES
            .iter()
            .filter(|&l| *l != self)
            .map(|&l| Language::to_open_graph_locale(l))
            .collect()
    }
}

impl TryFrom<&str> for Language {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Language::parse_from_bcp47_tag(value)
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let language = match self {
            Language::English => EN_US,
            Language::Spanish => ES_MX,
            Language::German => DE_DE,
            Language::French => FR_FR,
        };
        write!(f, "{}", language)
    }
}

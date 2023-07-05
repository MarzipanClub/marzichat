//! Internationalization module.
//!
//! This module contains all the translations for the copy used in the
//! app.

use {
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
    std::fmt::Display,
};

/// The various langauges that the application supports.
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "language"))]
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
pub const LANGUAGES: &[Language] = &[
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
        write!(f, "{}", self.as_bcp47_tag())
    }
}

/// Contains all translations for a given `Language`.
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

    translations!(
        create_an_account,
        "Create an account",
        "Crea una cuenta",
        "Ein Konto erstellen",
        "Créer un compte"
    );

    translations!(
        logo_of_the_letter_m,
        "Logo of the letter M",
        "Logo de la letra M",
        "Logo des Buchstabens M",
        "Le logo de la lettre M"
    );

    translations!(
        just_now,
        "just now",
        "justo ahora",
        "gerade eben",
        "à l'instant"
    );

    translations!(second, "second", "segundo", "Sekunde", "seconde");

    translations!(seconds, "seconds", "segundos", "Sekunden", "secondes");

    translations!(minute, "minute", "minuto", "Minute", "minute");

    translations!(minutes, "minutes", "minutos", "Minuten", "minutes");

    translations!(hour, "hour", "hora", "Stunde", "heure");

    translations!(hours, "hours", "horas", "Stunden", "heures");

    translations!(day, "day", "día", "Tag", "jour");

    translations!(days, "days", "días", "Tage", "jours");

    translations!(week, "week", "semana", "Woche", "semaine");

    translations!(weeks, "weeks", "semanas", "Wochen", "semaines");

    translations!(month, "month", "mes", "Monat", "mois");

    translations!(months, "months", "meses", "Monate", "mois");

    translations!(year, "year", "año", "Jahr", "an");

    translations!(years, "years", "años", "Jahre", "ans");

    /// Create a new `Translations` instance for the given `Language`.
    pub fn for_language(language: Language) -> Self {
        Self(language)
    }
}

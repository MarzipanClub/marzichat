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

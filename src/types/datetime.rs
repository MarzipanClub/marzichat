use {
    crate::internationalization::{Language, Translations},
    chrono::Duration,
};

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub trait CheckedAddition {
    /// Adds a duration to a datetime, returning `Error` if the result would
    /// overflow.
    fn add(&self, duration: &Duration) -> Result<DateTime, Error>;
}

impl CheckedAddition for DateTime {
    fn add(&self, duration: &Duration) -> Result<DateTime, Error> {
        match self.checked_add_signed(duration.to_owned()) {
            Some(result) => Ok(result),
            None => Err(Error {
                datetime: *self,
                duration: *duration,
            }),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Overflow when adding {duration:?} to {datetime:?})")]
pub struct Error {
    datetime: DateTime,
    duration: Duration,
}

enum Unit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

/// The threshold for the `just now` in seconds.
const JUST_NOW_THRESHOLD_SECONDS: i64 = 15;

/// Formats the datetime as a `x seconds/minutes/hours/weeks/months/years ago`.
pub fn ago(datetime: &DateTime, language: Language) -> String {
    let t = Translations::for_language(language);

    let duration = chrono::Utc::now() - *datetime;
    let seconds = duration.num_seconds();

    if seconds < JUST_NOW_THRESHOLD_SECONDS {
        t.just_now().to_owned()
    } else if seconds < 60 {
        ago_phrase(seconds, language, Unit::Second)
    } else if seconds < 60 * 60 {
        ago_phrase(seconds / 60, language, Unit::Minute)
    } else if seconds < 60 * 60 * 24 {
        ago_phrase(seconds / 60 / 60, language, Unit::Hour)
    } else if seconds < 60 * 60 * 24 * 7 {
        ago_phrase(seconds / 60 / 60 / 24, language, Unit::Day)
    } else if seconds < 60 * 60 * 24 * 7 * 4 {
        ago_phrase(seconds / 60 / 60 / 24 / 7, language, Unit::Week)
    } else if seconds < 60 * 60 * 24 * 7 * 4 * 12 {
        let mut value = (seconds as f64 / 60_f64 / 60_f64 / 24_f64 / 7_f64 / 4.348) as _;
        if value == 0 {
            value = 1;
        }
        ago_phrase(value, language, Unit::Month)
    } else {
        let mut value = (seconds as f64 / 60_f64 / 60_f64 / 24_f64 / 7_f64 / 4.348 / 12_f64) as _;
        if value == 0 {
            value = 1;
        }
        ago_phrase(value, language, Unit::Year)
    }
}

enum Number {
    Singular,
    Plural,
}

/// Returns the word for the unit in the provided language.
fn unit_word(language: Language, unit: Unit, number: Number) -> &'static str {
    let t = Translations::for_language(language);
    match unit {
        Unit::Second => match number {
            Number::Singular => t.second(),
            Number::Plural => t.seconds(),
        },
        Unit::Minute => match number {
            Number::Singular => t.minute(),
            Number::Plural => t.minutes(),
        },
        Unit::Hour => match number {
            Number::Singular => t.hour(),
            Number::Plural => t.hours(),
        },
        Unit::Day => match number {
            Number::Singular => t.day(),
            Number::Plural => t.days(),
        },
        Unit::Week => match number {
            Number::Singular => t.week(),
            Number::Plural => t.weeks(),
        },
        Unit::Month => match number {
            Number::Singular => t.month(),
            Number::Plural => t.months(),
        },
        Unit::Year => match number {
            Number::Singular => t.year(),
            Number::Plural => t.years(),
        },
    }
}

/// Formats the datetime as a `x seconds ago` in the provided language.
fn ago_phrase(value: i64, language: Language, unit: Unit) -> String {
    match language {
        Language::English => match value {
            1 => format!("1 {} ago", unit_word(language, unit, Number::Singular)),
            _ => format!("{value} {} ago", unit_word(language, unit, Number::Plural)),
        },
        Language::Spanish => match value {
            1 => format!("hace 1 {}", unit_word(language, unit, Number::Singular)),
            _ => format!("hace {value} {}", unit_word(language, unit, Number::Plural)),
        },
        Language::German => match value {
            1 => format!("vor 1 {}", unit_word(language, unit, Number::Singular)),
            _ => format!("vor {value} {}", unit_word(language, unit, Number::Plural)),
        },
        Language::French => match value {
            1 => format!("il y a 1 {}", unit_word(language, unit, Number::Singular)),
            _ => format!(
                "il y a {value} {}",
                unit_word(language, unit, Number::Plural)
            ),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_10_seconds_ago() {
        let datetime = chrono::Utc::now() - Duration::seconds(10);
        assert_eq!(ago(&datetime, Language::English), "just now");
        assert_eq!(ago(&datetime, Language::Spanish), "justo ahora");
        assert_eq!(ago(&datetime, Language::German), "gerade eben");
        assert_eq!(ago(&datetime, Language::French), "à l'instant");
    }

    #[test]
    fn test_30_seconds_ago() {
        let datetime = chrono::Utc::now() - Duration::seconds(30);
        assert_eq!(ago(&datetime, Language::English), "30 seconds ago");
        assert_eq!(ago(&datetime, Language::Spanish), "hace 30 segundos");
        assert_eq!(ago(&datetime, Language::German), "vor 30 Sekunden");
        assert_eq!(ago(&datetime, Language::French), "il y a 30 secondes");
    }

    #[test]
    fn test_1_day_ago() {
        let jitter = Duration::seconds(7);
        let datetime = chrono::Utc::now() - Duration::days(1) - jitter;
        assert_eq!(ago(&datetime, Language::English), "1 day ago");
        assert_eq!(ago(&datetime, Language::Spanish), "hace 1 día");
        assert_eq!(ago(&datetime, Language::German), "vor 1 Tag");
        assert_eq!(ago(&datetime, Language::French), "il y a 1 jour");
    }

    #[test]
    fn test_6_day_ago() {
        let jitter = Duration::hours(23);
        let datetime = chrono::Utc::now() - Duration::days(6) - jitter;
        assert_eq!(ago(&datetime, Language::English), "6 days ago");
        assert_eq!(ago(&datetime, Language::Spanish), "hace 6 días");
        assert_eq!(ago(&datetime, Language::German), "vor 6 Tage");
        assert_eq!(ago(&datetime, Language::French), "il y a 6 jours");
    }

    #[test]
    fn test_1_week_ago() {
        let jitter = Duration::days(4);
        let datetime = chrono::Utc::now() - Duration::weeks(1) - jitter;
        assert_eq!(ago(&datetime, Language::English), "1 week ago");
        assert_eq!(ago(&datetime, Language::Spanish), "hace 1 semana");
        assert_eq!(ago(&datetime, Language::German), "vor 1 Woche");
        assert_eq!(ago(&datetime, Language::French), "il y a 1 semaine");
    }

    #[test]
    fn test_1_months_ago() {
        let jitter = Duration::hours(1);
        let datetime = chrono::Utc::now() - Duration::hours(30 * 24) - jitter;
        assert_eq!(ago(&datetime, Language::English), "1 month ago");
        assert_eq!(ago(&datetime, Language::Spanish), "hace 1 mes");
        assert_eq!(ago(&datetime, Language::German), "vor 1 Monat");
        assert_eq!(ago(&datetime, Language::French), "il y a 1 mois");
    }

    #[test]
    fn test_6_months_ago() {
        let jitter = Duration::hours(1);
        let datetime =
            chrono::Utc::now() - Duration::hours((30.437_f64 * 24_f64 * 6_f64) as _) - jitter;
        assert_eq!(ago(&datetime, Language::English), "6 months ago");
        assert_eq!(ago(&datetime, Language::Spanish), "hace 6 meses");
        assert_eq!(ago(&datetime, Language::German), "vor 6 Monaten");
        assert_eq!(ago(&datetime, Language::French), "il y a 6 mois");
    }
}

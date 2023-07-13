//! Defines types used throughout the project.

use {
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

pub mod datetime;
pub mod email;
pub mod password;
pub mod username;

pub use {datetime::DateTime, email::Email, password::Password, username::Username};

/// A user id.
#[cfg_attr(feature = "sqlx", derive(sqlx::Type), sqlx(transparent))]
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Serialize, Deserialize, From, Display, Hash,
)]
#[from(forward)]
pub struct UserId(pub Uuid);

pub mod validation {
    //! # Validation module.
    //!
    //! This module contains various validation types and traits.

    use {
        derive_more::{Deref, Display},
        serde::{Deserialize, Serialize},
        std::{fmt, slice::Iter, vec::IntoIter},
    };

    /// A validator for the various types.
    pub struct Validator<E>(Vec<E>);

    impl<E> Validator<E> {
        /// Creates a validator.
        pub fn new() -> Self {
            Self(Vec::new())
        }

        /// Add the invalidity if the condition is true.
        pub fn invalid_if(mut self, condition: bool, invalidity: E) -> Self
        where
            E: PartialEq + Eq,
        {
            if condition {
                self.0.push(invalidity);
            }
            self
        }

        /// Validates the field.
        pub fn validate_field<V, B, T>(mut self, field: &V, violation: B) -> Self
        where
            V: Validate<Violation = T>,
            T: PartialEq + Eq + fmt::Debug + fmt::Display,
            B: Fn(Violations<T>) -> E,
            E: PartialEq + Eq + fmt::Debug + fmt::Display,
        {
            let validation = field.validate();
            match validation {
                Ok(()) => (),
                Err(invalidities) => {
                    self.0.push(violation(invalidities));
                }
            };
            self
        }
    }

    impl<E> Default for Validator<E> {
        fn default() -> Self {
            Self::new()
        }
    }

    /// The invalidities found after validation.
    #[derive(
        thiserror::Error, Debug, Display, PartialEq, Eq, Clone, Serialize, Deserialize, Deref,
    )]
    #[deref(forward)]
    #[display(fmt = "{:#?}", _0)]
    pub struct Violations<E>(Vec<E>)
    where
        E: PartialEq + Eq + fmt::Debug + fmt::Display;

    impl<E> Violations<E>
    where
        E: PartialEq + Eq + fmt::Debug + fmt::Display,
    {
        /// Returns true if the invalidity is contained in the vector.
        pub fn contains(&self, invalidity: &E) -> bool {
            self.0.contains(invalidity)
        }

        /// An iterator over the invalidities.
        pub fn iter(&self) -> Iter<'_, E> {
            self.0.iter()
        }
    }

    impl<E: PartialEq + Eq + fmt::Debug + fmt::Display> IntoIterator for Violations<E> {
        type IntoIter = IntoIter<Self::Item>;
        type Item = E;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    impl<E> From<Validator<E>> for Result<(), Violations<E>>
    where
        E: PartialEq + Eq + fmt::Debug + fmt::Display,
    {
        fn from(validator: Validator<E>) -> Self {
            if validator.0.is_empty() {
                Ok(())
            } else {
                Err(Violations(validator.0.into_iter().collect()))
            }
        }
    }

    /// Denotes a type that can be validated.
    pub trait Validate {
        type Violation: PartialEq + Eq + fmt::Debug + fmt::Display;

        /// Validates the type.
        fn validate(&self) -> Result<(), Violations<Self::Violation>> {
            Ok(())
        }

        fn is_valid(&self) -> bool {
            self.validate().is_ok()
        }
    }

    impl<T> Validate for Option<T>
    where
        T: Validate,
    {
        type Violation = T::Violation;

        fn validate(&self) -> Result<(), Violations<Self::Violation>> {
            match self {
                Some(value) => value.validate(),
                None => Ok(()),
            }
        }
    }

    /// Unit struct an invalidity that is always valid.
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Display)]
    pub struct NeverInvalid;
}

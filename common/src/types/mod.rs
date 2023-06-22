//! Defines types used throughout the project.

pub mod account;
pub mod datetime;
pub mod email;
pub mod password;
pub mod username;

mod password_hash;

pub use {
    datetime::DateTime, email::Email, password::Password, password_hash::PasswordHash,
    username::Username,
};

pub mod validation {
    //! # Validation module.
    //!
    //! This module contains various validation types and traits.

    use {
        core::hash::Hash,
        derive_more::{Deref, Display},
        serde::{Deserialize, Serialize},
        std::{
            collections::{
                hash_set::{IntoIter, Iter},
                HashSet,
            },
            fmt,
        },
    };

    /// A validator for the various types.
    pub struct Validator<E>(HashSet<E>);

    impl<E> Validator<E> {
        /// Creates a validator.
        pub fn new() -> Self {
            Self(HashSet::new())
        }

        /// Add the invalidity if the condition is true.
        pub fn invalid_if(mut self, condition: bool, invalidity: E) -> Self
        where
            E: Hash + PartialEq + Eq,
        {
            if condition {
                self.0.insert(invalidity);
            }
            self
        }

        /// Validates the field.
        pub fn validate_field<V, B, T>(mut self, field: &V, variant: B) -> Self
        where
            V: Validate<Invalidity = T>,
            T: Hash + PartialEq + Eq + fmt::Debug + fmt::Display,
            B: Fn(Invalidities<T>) -> E,
            E: Hash + PartialEq + Eq + fmt::Debug + fmt::Display,
        {
            let validation = field.validate();
            match validation {
                Ok(()) => (),
                Err(invalidities) => {
                    self.0.insert(variant(invalidities));
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
    pub struct Invalidities<E>(pub HashSet<E>)
    where
        E: Hash + PartialEq + Eq + fmt::Debug + fmt::Display;

    impl<E> Invalidities<E>
    where
        E: Hash + PartialEq + Eq + fmt::Debug + fmt::Display,
    {
        /// Returns true if the invalidity is contained in the set.
        pub fn contains(&self, invalidity: &E) -> bool {
            self.0.contains(invalidity)
        }

        /// An iterator over the invalidities.
        pub fn iter(&self) -> Iter<'_, E> {
            self.0.iter()
        }

        /// An owning iterator over the invalidities.
        pub fn into_iter(self) -> IntoIter<E> {
            self.0.into_iter()
        }
    }

    impl<E> From<Validator<E>> for Result<(), Invalidities<E>>
    where
        E: Hash + PartialEq + Eq + fmt::Debug + fmt::Display,
    {
        fn from(validator: Validator<E>) -> Self {
            if validator.0.is_empty() {
                Ok(())
            } else {
                Err(Invalidities(validator.0))
            }
        }
    }

    /// Denotes a type that can be validated.
    pub trait Validate {
        type Invalidity: Hash + PartialEq + Eq + fmt::Debug + fmt::Display;

        /// Validates the type.
        fn validate(&self) -> Result<(), Invalidities<Self::Invalidity>> {
            Ok(())
        }

        fn is_valid(&self) -> bool {
            self.validate().is_ok()
        }
    }

    /// Unit struct an invalidity that is always valid.
    #[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display)]
    pub struct NeverInvalid;
}

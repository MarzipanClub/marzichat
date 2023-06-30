//! Exponential backoff.

use std::{ops::Deref, time::Duration};

/// A wrapper around `Duration` that implements exponential backoff.
#[derive(Debug, Clone, Copy)]
pub struct Backoff(Duration);

/// The initial backoff duration.
const INITIAL: Duration = Duration::from_millis(600);

/// The backoff multiplier.
const MULTIPLIER: f64 = 1.2;

impl Backoff {
    /// Creates a new backoff with the initial value and multiplier.
    pub fn new() -> Self {
        Self(INITIAL)
    }

    /// Resets the backoff to the initial value.
    pub fn reset(&mut self) {
        self.0 = INITIAL;
    }

    /// Exponentially increases the backoff.
    pub fn increase(&mut self) {
        self.0 = self.0.mul_f64(MULTIPLIER);
        log::debug!("increased backoff to {:?}", self.0);
    }
}

impl Deref for Backoff {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

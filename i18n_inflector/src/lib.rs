//! Multilingual noun singularization and pluralization.
//!
//! `i18n_inflector` provides functions to convert between singular and plural forms of nouns across
//! ISO 639 two-letter language codes.
//!
//! # Quick Start
//!
//! ```
//! use i18n_inflector::{singularize, pluralize};
//!
//! // English
//! assert_eq!(singularize("en", "users").unwrap(), "user");
//! assert_eq!(singularize("en", "categories").unwrap(), "category");
//!
//! let plurals = pluralize("en", "user").unwrap();
//! assert!(plurals.iter().any(|v| v == "users"));
//!
//! // Spanish
//! assert_eq!(singularize("es", "ciudades").unwrap(), "ciudad");
//!
//! // French
//! assert_eq!(singularize("fr", "journaux").unwrap(), "journal");
//!
//! // Japanese
//! assert_eq!(singularize("ja", "user").unwrap(), "user");
//!
//! // Unsupported locale returns an error
//! assert!(singularize("xx", "users").is_err());
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

mod error;
mod languages;
mod pluralize;
mod singularize;

pub use error::{Error, Result};
pub use pluralize::pluralize;
pub use singularize::singularize;

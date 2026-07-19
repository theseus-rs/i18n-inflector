//! Correctness-first dictionary-form noun inflection for every ISO Set 1 language code.
//!
//! The input is a dictionary lemma, not an arbitrary already-inflected word. Successful results
//! contain only curated embedded entries or forms produced by an explicitly selected productive
//! class.
//!
//! ```
//! use i18n_inflector::{InflectionRequest, LexicalClassId, language_profile};
//!
//! let english = language_profile("en-US")?;
//! assert_eq!(
//!     english.inflect(InflectionRequest::plural("child"))?.primary(),
//!     "children"
//! );
//! assert_eq!(
//!     english
//!         .inflect(
//!             InflectionRequest::plural("project")
//!                 .lexical_class(LexicalClassId::new("regular-s"))
//!         )?
//!         .primary(),
//!     "projects"
//! );
//! # Ok::<(), i18n_inflector::Error>(())
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

mod error;
mod features;
mod languages;
mod locale;
mod profile;
mod registry;

pub use error::{Error, Result};
pub use features::{
    Animacy, Countability, Gender, InflectionRequest, LexicalClassId, Number, SelectorKind,
};
pub use profile::{InflectedForms, LanguageCapabilities, LanguageProfile, LexicalClassSpec};

/// Returns the language profile selected by a BCP 47 language identifier.
///
/// A region subtag is accepted and ignored when no region-specific profile is needed. An explicit
/// script must have a registered profile and is never silently replaced with another script.
///
/// # Errors
///
/// Returns [`Error::InvalidLocale`] for malformed identifiers and [`Error::UnsupportedLocale`] for
/// well-formed identifiers without a supported language or script profile.
pub fn language_profile(locale: &str) -> Result<&'static LanguageProfile> {
    locale::resolve(locale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_entry_point_resolves_profiles() {
        assert_eq!(
            language_profile("en").map(LanguageProfile::language),
            Ok("en")
        );
    }
}

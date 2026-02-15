//! Hindi (hi) inflection rules.
//!
//! Also used for Divehi (dv).

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "hi",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Indic noun to its singular form.
///
/// Handles both Latin transliteration suffixes (`-on`, `-en`) and native Devanagari plural suffix
/// (`-ों` / `-ें`).
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    // Native Devanagari: -ों (anusvara + visarga-like plural)
    if let Some(stem) = name.strip_suffix("ों")
        && !stem.is_empty()
    {
        return format!("{stem}ा").into();
    }
    // Native Devanagari: -ें
    if let Some(stem) = name.strip_suffix("ें")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    // Latin transliteration
    if let Some(stem) = name.strip_suffix("on")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("en")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Indic noun.
///
/// Generates both Latin transliteration and native Devanagari plural forms.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = vec![format!("{name}on").into(), format!("{name}en").into()];

    // If the word ends with 'ा' (Devanagari aa), generate -ों form
    if let Some(stem) = name.strip_suffix('ा') {
        candidates.push(format!("{stem}ों").into());
    }
    // Also generate -ें form
    candidates.push(format!("{name}ें").into());

    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("upyogakartaon"), "upyogakarta");
        assert_eq!(singularize("shikshaken"), "shikshak");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("upyogakarta"), "upyogakarta");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("on"), "on");
        assert_eq!(singularize("en"), "en");
    }

    #[test]
    fn test_singularize_devanagari() {
        assert_eq!(singularize("लड़कों"), "लड़का");
        assert_eq!(singularize("किताबें"), "किताब");
    }

    #[test]
    fn test_singularize_devanagari_suffix_only() {
        // Single suffix character shouldn't be stripped
        assert_eq!(singularize("ों"), "ों");
        assert_eq!(singularize("ें"), "ें");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("upyogakarta");
        assert!(result.iter().any(|v| v == "upyogakartaon"));
        assert!(result.iter().any(|v| v == "upyogakartaen"));
    }

    #[test]
    fn test_pluralize_devanagari() {
        let result = pluralize("लड़का");
        assert!(result.iter().any(|v| v == "लड़कों"));
        assert!(result.iter().any(|v| v == "लड़काें"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert!(result.len() >= 2);
    }
}

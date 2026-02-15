//! Bengali (bn) inflection rules.
//!
//! Handles both Latin transliteration and native Bengali script.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "bn",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Bengali noun to its singular form.
///
/// Handles Latin transliteration suffixes (`-on`, `-en`) and native Bengali plural suffixes
/// (`-গুলি`, `-গুলো`, `-সমূহ`, `-দের`, `-রা`).
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    // Native Bengali suffixes (longer suffixes first)
    for suffix in &["গুলি", "গুলো", "সমূহ", "দের", "রা"] {
        if let Some(stem) = name.strip_suffix(suffix)
            && !stem.is_empty()
        {
            return Cow::Borrowed(stem);
        }
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

/// Returns a list of possible plural forms for a Bengali noun.
///
/// Generates both Latin transliteration and native Bengali plural forms.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}on").into(),
        format!("{name}en").into(),
        format!("{name}গুলি").into(),
        format!("{name}গুলো").into(),
        format!("{name}সমূহ").into(),
        format!("{name}দের").into(),
        format!("{name}রা").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_latin() {
        assert_eq!(singularize("upyogakartaon"), "upyogakarta");
        assert_eq!(singularize("shikshaken"), "shikshak");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("upyogakarta"), "upyogakarta");
    }

    #[test]
    fn test_singularize_suffix_only() {
        assert_eq!(singularize("on"), "on");
        assert_eq!(singularize("en"), "en");
    }

    #[test]
    fn test_singularize_bengali() {
        assert_eq!(singularize("বইগুলি"), "বই");
        assert_eq!(singularize("বইগুলো"), "বই");
        assert_eq!(singularize("ছাত্রদের"), "ছাত্র");
        assert_eq!(singularize("ছাত্ররা"), "ছাত্র");
        assert_eq!(singularize("বইসমূহ"), "বই");
    }

    #[test]
    fn test_singularize_bengali_suffix_only() {
        assert_eq!(singularize("গুলি"), "গুলি");
        assert_eq!(singularize("গুলো"), "গুলো");
        assert_eq!(singularize("রা"), "রা");
    }

    #[test]
    fn test_pluralize_latin() {
        let result = pluralize("upyogakarta");
        assert!(result.iter().any(|v| v == "upyogakartaon"));
        assert!(result.iter().any(|v| v == "upyogakartaen"));
    }

    #[test]
    fn test_pluralize_bengali() {
        let result = pluralize("বই");
        assert!(result.iter().any(|v| v == "বইগুলি"));
        assert!(result.iter().any(|v| v == "বইগুলো"));
        assert!(result.iter().any(|v| v == "বইসমূহ"));
        assert!(result.iter().any(|v| v == "বইদের"));
        assert!(result.iter().any(|v| v == "বইরা"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert!(result.len() >= 2);
    }
}

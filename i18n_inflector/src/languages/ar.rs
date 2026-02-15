//! Arabic (ar) inflection rules.
//!
//! Also used for Hebrew (he) and Yiddish (yi).

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ar",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Semitic noun to its singular form.
///
/// Handles both Latin transliteration suffixes (`-im`, `-ot`, `-at`, `-in`) and native Arabic
/// plural suffixes (`-ون`, `-ين`, `-ات`).
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    // Native Arabic plural suffixes
    for suffix in &["ون", "ين", "ات"] {
        if let Some(stem) = name.strip_suffix(suffix)
            && !stem.is_empty()
        {
            return Cow::Borrowed(stem);
        }
    }
    // Latin transliteration
    if let Some(stem) = name.strip_suffix("im")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ot")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("at")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("in")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Semitic noun.
///
/// Generates both Latin transliteration and native Arabic plural forms.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}im").into(),
        format!("{name}ot").into(),
        format!("{name}at").into(),
        format!("{name}in").into(),
        format!("{name}ون").into(),
        format!("{name}ين").into(),
        format!("{name}ات").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_latin() {
        assert_eq!(singularize("meshtatfim"), "meshtatf");
        assert_eq!(singularize("mustahdimin"), "mustahdim");
        assert_eq!(singularize("produktin"), "produkt");
        assert_eq!(singularize("mishpachot"), "mishpach");
        assert_eq!(singularize("banat"), "ban");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("meshtatef"), "meshtatef");
    }

    #[test]
    fn test_singularize_arabic() {
        assert_eq!(singularize("معلمون"), "معلم");
        assert_eq!(singularize("معلمين"), "معلم");
        assert_eq!(singularize("طالبات"), "طالب");
    }

    #[test]
    fn test_singularize_arabic_suffix_only() {
        assert_eq!(singularize("ون"), "ون");
        assert_eq!(singularize("ين"), "ين");
        assert_eq!(singularize("ات"), "ات");
    }

    #[test]
    fn test_pluralize_latin() {
        let result = pluralize("meshtatef");
        assert!(result.iter().any(|v| v == "meshtatefim"));
        assert!(result.iter().any(|v| v == "meshtatefot"));
        assert!(result.iter().any(|v| v == "meshtatefat"));
        assert!(result.iter().any(|v| v == "meshtatefin"));
    }

    #[test]
    fn test_pluralize_arabic() {
        let result = pluralize("معلم");
        assert!(result.iter().any(|v| v == "معلمون"));
        assert!(result.iter().any(|v| v == "معلمين"));
        assert!(result.iter().any(|v| v == "معلمات"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}

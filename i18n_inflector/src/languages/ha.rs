//! Hausa (ha) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ha",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Hausa noun to its singular form.
///
/// Handles `-una`, `-oci`, `-ai`, and `-i` plural suffixes.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("una")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("oci")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ai")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Hausa noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}una").into(),
        format!("{name}oci").into(),
        format!("{name}ai").into(),
        format!("{name}i").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_una_suffix() {
        assert_eq!(singularize("gidauna"), "gida");
    }

    #[test]
    fn test_singularize_oci_suffix() {
        assert_eq!(singularize("malamoci"), "malam");
    }

    #[test]
    fn test_singularize_ai_suffix() {
        assert_eq!(singularize("litattafai"), "litattaf");
    }

    #[test]
    fn test_singularize_i_suffix() {
        assert_eq!(singularize("yari"), "yar");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("gida"), "gida");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "una" -> strip "una" -> "" empty -> skip; "oci" no; "ai" no; 'i' no -> "una"
        assert_eq!(singularize("una"), "una");
        // "oci" -> strip "una" no; strip "oci" -> "" empty -> skip; "ai" no; 'i' -> "oc"
        assert_eq!(singularize("oci"), "oc");
        // "ai" -> strip "una" no; strip "oci" no; strip "ai" -> "" empty -> skip; 'i' -> "a"
        assert_eq!(singularize("ai"), "a");
        assert_eq!(singularize("i"), "i");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("littafi");
        assert_eq!(result.len(), 4);
        assert!(result.iter().any(|v| v == "littafiuna"));
        assert!(result.iter().any(|v| v == "littafioci"));
        assert!(result.iter().any(|v| v == "littafiai"));
        assert!(result.iter().any(|v| v == "littafii"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 4);
    }
}

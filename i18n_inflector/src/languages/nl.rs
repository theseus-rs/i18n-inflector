//! Dutch (nl) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Dutch noun to its singular form.
///
/// Handles `-en`, `-'s`, and `-s` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("nl", "klanten").unwrap(), "klant");
/// assert_eq!(singularize("nl", "producten").unwrap(), "product");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("en")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("'s")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('s')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Dutch noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("nl", "klant").unwrap();
/// assert!(result.iter().any(|v| v == "klanten"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}en").into(),
        format!("{name}s").into(),
        format!("{name}'s").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_en_suffix() {
        assert_eq!(singularize("klanten"), "klant");
        assert_eq!(singularize("producten"), "product");
        assert_eq!(singularize("boeken"), "boek");
    }

    #[test]
    fn test_singularize_apostrophe_s_suffix() {
        assert_eq!(singularize("auto's"), "auto");
    }

    #[test]
    fn test_singularize_s_suffix() {
        assert_eq!(singularize("hotels"), "hotel");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("klant"), "klant");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "en" with empty stem falls through all branches, returns as-is
        assert_eq!(singularize("en"), "en");
        // "'s" with empty stem falls through to 's' branch: stem "'" non-empty -> "'"
        assert_eq!(singularize("'s"), "'");
        // "s" with empty stem falls through -> returns "s"
        assert_eq!(singularize("s"), "s");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("klant");
        assert_eq!(result.len(), 3);
        assert!(result.iter().any(|v| v == "klanten"));
        assert!(result.iter().any(|v| v == "klants"));
        assert!(result.iter().any(|v| v == "klant's"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 3);
    }
}

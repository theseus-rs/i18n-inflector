//! Guarani (gn) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Guarani noun to its singular form.
///
/// Guarani uses `-kuéra` as the plural suffix.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("gn", "mitãkuéra").unwrap(), "mitã");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("kuéra")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Guarani noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("gn", "mitã").unwrap();
/// assert!(result.iter().any(|v| v == "mitãkuéra"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}kuéra").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_kuera_suffix() {
        assert_eq!(singularize("mitãkuéra"), "mitã");
        assert_eq!(singularize("jagkuéra"), "jag");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("mitã"), "mitã");
    }

    #[test]
    fn test_singularize_suffix_only_input() {
        assert_eq!(singularize("kuéra"), "kuéra");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("mitã");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "mitãkuéra"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}

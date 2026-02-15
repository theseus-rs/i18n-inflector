//! Maltese (mt) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Maltese noun to its singular form.
///
/// Handles `-ijiet` and `-i` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("mt", "utenti").unwrap(), "utent");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ijiet")
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

/// Returns a list of possible plural forms for a Maltese noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("mt", "utent").unwrap();
/// assert!(result.iter().any(|v| v == "utenti"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}i").into(), format!("{name}ijiet").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("utenti"), "utent");
        assert_eq!(singularize("prodottijiet"), "prodott");
        assert_eq!(singularize("klienti"), "klient");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("utent"), "utent");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("utent");
        assert!(result.iter().any(|v| v == "utenti"));
        assert!(result.iter().any(|v| v == "utentijiet"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}

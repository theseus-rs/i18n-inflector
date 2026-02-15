//! Finnish (fi) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Finnish noun to its singular form.
///
/// Finnish nominative plurals typically end in `-t`.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("fi", "tuotteet").unwrap(), "tuottee");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('t')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Finnish noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("fi", "tuote").unwrap();
/// assert!(result.iter().any(|v| v == "tuotet"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}t").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("tuotteet"), "tuottee");
        assert_eq!(singularize("asiakkaat"), "asiakkaa");
        assert_eq!(singularize("tietokannat"), "tietokanna");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("tuote"), "tuote");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("tuote");
        assert!(result.iter().any(|v| v == "tuotet"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}

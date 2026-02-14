//! Romanian (ro) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Romanian noun to its singular form.
///
/// Handles `-i`, `-e`, and `-uri` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("ro", "utilizatori").unwrap(), "utilizator");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("uri")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('e')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Romanian noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("ro", "utilizator").unwrap();
/// assert!(result.iter().any(|v| v == "utilizatori"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}i").into(),
        format!("{name}e").into(),
        format!("{name}uri").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("utilizatori"), "utilizator");
        assert_eq!(singularize("produse"), "produs");
        assert_eq!(singularize("locuri"), "loc");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("utilizator"), "utilizator");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("utilizator");
        assert!(result.iter().any(|v| v == "utilizatori"));
        assert!(result.iter().any(|v| v == "utilizatore"));
        assert!(result.iter().any(|v| v == "utilizatoruri"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}

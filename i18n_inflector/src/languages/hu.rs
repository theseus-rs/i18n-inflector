//! Hungarian (hu) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Hungarian noun to its singular form.
///
/// Hungarian plurals end in `-k`, often with a linking vowel (`-ok`, `-ek`,
/// `-ök`).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("hu", "felhasznalok").unwrap(), "felhasznalo");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('k')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Hungarian noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("hu", "felhasznalo").unwrap();
/// assert!(result.iter().any(|v| v == "felhasznalok"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}k").into(),
        format!("{name}ok").into(),
        format!("{name}ek").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("felhasznalok"), "felhasznalo");
        assert_eq!(singularize("termekek"), "termeke");
        assert_eq!(singularize("rendelesek"), "rendelese");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("felhasznalo"), "felhasznalo");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("felhasznalo");
        assert!(result.iter().any(|v| v == "felhasznalok"));
        assert!(result.iter().any(|v| v == "felhasznalook"));
        assert!(result.iter().any(|v| v == "felhasznaloek"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}

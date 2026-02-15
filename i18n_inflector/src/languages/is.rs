//! Icelandic (is) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Icelandic noun to its singular form.
///
/// Handles `-ar`, `-ir`, and `-ur` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("is", "notendur").unwrap(), "notend");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ar")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ir")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ur")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Icelandic noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("is", "notend").unwrap();
/// assert!(result.iter().any(|v| v == "notendur"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}ar").into(),
        format!("{name}ir").into(),
        format!("{name}ur").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("notendur"), "notend");
        assert_eq!(singularize("baekur"), "baek");
        assert_eq!(singularize("stolar"), "stol");
        assert_eq!(singularize("gestir"), "gest");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("notend"), "notend");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("notend");
        assert!(result.iter().any(|v| v == "notendur"));
        assert!(result.iter().any(|v| v == "notendar"));
        assert!(result.iter().any(|v| v == "notendir"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}

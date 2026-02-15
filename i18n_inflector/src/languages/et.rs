//! Estonian (et) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Estonian noun to its singular form.
///
/// Estonian nominative plurals end in `-d`.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("et", "kasutajad").unwrap(), "kasutaja");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('d')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Estonian noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("et", "kasutaja").unwrap();
/// assert!(result.iter().any(|v| v == "kasutajad"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}d").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("kasutajad"), "kasutaja");
        assert_eq!(singularize("tooted"), "toote");
        assert_eq!(singularize("lauad"), "laua");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("kasutaja"), "kasutaja");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("kasutaja");
        assert!(result.iter().any(|v| v == "kasutajad"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}

//! Japanese (ja) inflection rules.
//!
//! Japanese does not mark plurality on nouns; words are returned unchanged. Also used for
//! Chinese (zh), Korean (ko), Thai (th), Vietnamese (vi), Javanese (jv), Malay (ms), and
//! Georgian (ka).

use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;

/// Returns the word unchanged since the language has no morphological plural.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("ja", "user").unwrap(), "user");
/// assert_eq!(singularize("zh", "user").unwrap(), "user");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    Cow::Borrowed(name)
}

/// Returns the word unchanged since the language has no morphological plural.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("ja", "user").unwrap();
/// assert_eq!(result, vec!["user"]);
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![Cow::Borrowed(name)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("user"), "user");
        assert_eq!(singularize("product"), "product");
        assert_eq!(singularize(""), "");
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize("user"), vec!["user"]);
        assert_eq!(pluralize("product"), vec!["product"]);
        assert_eq!(pluralize(""), vec![""]);
    }
}

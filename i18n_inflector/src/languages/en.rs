//! English (en) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural English noun to its singular form.
///
/// Handles regular `-s` plurals, `-es` plurals (for words ending in `x`, `ch`, `sh`, `z`), `-ies`
/// plurals, and `-sses` plurals.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("en", "users").unwrap(), "user");
/// assert_eq!(singularize("en", "categories").unwrap(), "category");
/// assert_eq!(singularize("en", "boxes").unwrap(), "box");
/// assert_eq!(singularize("en", "addresses").unwrap(), "address");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ies")
        && !stem.is_empty()
    {
        return format!("{stem}y").into();
    }
    if let Some(stem) = name.strip_suffix("sses")
        && !stem.is_empty()
    {
        return format!("{stem}ss").into();
    }
    if let Some(stem) = name.strip_suffix("es") {
        if stem.ends_with('x')
            || stem.ends_with("ch")
            || stem.ends_with("sh")
            || stem.ends_with('z')
        {
            return Cow::Borrowed(stem);
        }
        return Cow::Borrowed(name.strip_suffix('s').unwrap_or(name));
    }
    if let Some(stem) = name.strip_suffix('s')
        && !stem.is_empty()
        && !stem.ends_with('s')
        && !name.ends_with("us")
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an English noun.
///
/// Generates candidates with `-s`, `-es`, and `-ies` (for words ending in `y`)
/// suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("en", "user").unwrap();
/// assert!(result.iter().any(|v| v == "users"));
///
/// let result = pluralize("en", "category").unwrap();
/// assert!(result.iter().any(|v| v == "categories"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = vec![format!("{name}s").into(), format!("{name}es").into()];
    if let Some(stem) = name.strip_suffix('y')
        && !stem.is_empty()
    {
        candidates.push(format!("{stem}ies").into());
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_regular_s() {
        assert_eq!(singularize("users"), "user");
        assert_eq!(singularize("products"), "product");
        assert_eq!(singularize("accounts"), "account");
        assert_eq!(singularize("orders"), "order");
        assert_eq!(singularize("items"), "item");
        assert_eq!(singularize("roles"), "role");
    }

    #[test]
    fn test_singularize_es_suffix() {
        assert_eq!(singularize("boxes"), "box");
        assert_eq!(singularize("matches"), "match");
        assert_eq!(singularize("dishes"), "dish");
        assert_eq!(singularize("addresses"), "address");
        assert_eq!(singularize("buzzes"), "buzz");
        assert_eq!(singularize("watches"), "watch");
        assert_eq!(singularize("bushes"), "bush");
    }

    #[test]
    fn test_singularize_ies_suffix() {
        assert_eq!(singularize("categories"), "category");
        assert_eq!(singularize("companies"), "company");
        assert_eq!(singularize("countries"), "country");
        assert_eq!(singularize("stories"), "story");
        assert_eq!(singularize("cities"), "city");
        assert_eq!(singularize("policies"), "policy");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("user"), "user");
        assert_eq!(singularize("status"), "status");
        assert_eq!(singularize("bus"), "bus");
        assert_eq!(singularize("class"), "class");
    }

    #[test]
    fn test_singularize_empty_and_single_char() {
        assert_eq!(singularize(""), "");
        assert_eq!(singularize("s"), "s");
        assert_eq!(singularize("x"), "x");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("user");
        assert!(result.iter().any(|v| v == "users"));
        assert!(result.iter().any(|v| v == "useres"));

        let result = pluralize("category");
        assert!(result.iter().any(|v| v == "categories"));

        let result = pluralize("box");
        assert!(result.iter().any(|v| v == "boxs"));
        assert!(result.iter().any(|v| v == "boxes"));
    }

    #[test]
    fn test_pluralize_word_ending_in_y() {
        let result = pluralize("company");
        assert!(result.iter().any(|v| v == "companies"));

        let result = pluralize("story");
        assert!(result.iter().any(|v| v == "stories"));
    }

    #[test]
    fn test_pluralize_empty() {
        let result = pluralize("");
        assert!(result.iter().any(|v| v == "s"));
    }
}

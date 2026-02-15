//! English (en) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

use crate::language_rules::LanguageRuleSet;

/// Irregular plural -> singular mappings.
static EXCEPTIONS_PLURAL_TO_SINGULAR: &[(&str, &str)] = &[
    ("children", "child"),
    ("geese", "goose"),
    ("men", "man"),
    ("mice", "mouse"),
    ("oxen", "ox"),
    ("people", "person"),
    ("teeth", "tooth"),
    ("women", "woman"),
];

/// Irregular singular -> plural mappings.
static EXCEPTIONS_SINGULAR_TO_PLURAL: &[(&str, &str)] = &[
    ("child", "children"),
    ("goose", "geese"),
    ("man", "men"),
    ("mouse", "mice"),
    ("ox", "oxen"),
    ("person", "people"),
    ("tooth", "teeth"),
    ("woman", "women"),
];

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "en",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural English noun to its singular form.
///
/// Handles irregular exceptions (e.g., children -> child, oxen -> ox) before falling back to
/// regular suffix rules: `-s`, `-es`, `-ies`, `-sses`.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    // Check exceptions first
    for &(plural, singular) in EXCEPTIONS_PLURAL_TO_SINGULAR {
        if name == plural {
            return Cow::Borrowed(singular);
        }
    }

    // Algorithmic suffix rules
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
/// Checks irregular exceptions first (e.g., child -> children), then generates candidates with
/// `-s`, `-es`, and `-ies` suffixes.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    // Check exceptions first
    for &(singular, plural) in EXCEPTIONS_SINGULAR_TO_PLURAL {
        if name == singular {
            return vec![Cow::Borrowed(plural)];
        }
    }

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
    use crate::language_rules::LanguageRules;

    #[test]
    fn test_rules_language() {
        assert_eq!(RULES.language(), "en");
    }

    #[test]
    fn test_rules_singularize() {
        assert_eq!(RULES.singularize("users"), "user");
        assert_eq!(RULES.singularize("children"), "child");
    }

    #[test]
    fn test_rules_pluralize() {
        let result = RULES.pluralize("user");
        assert!(result.iter().any(|v| v == "users"));
        let result = RULES.pluralize("child");
        assert_eq!(result, vec!["children"]);
    }

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
    fn test_singularize_exceptions() {
        assert_eq!(singularize("children"), "child");
        assert_eq!(singularize("oxen"), "ox");
        assert_eq!(singularize("men"), "man");
        assert_eq!(singularize("women"), "woman");
        assert_eq!(singularize("mice"), "mouse");
        assert_eq!(singularize("geese"), "goose");
        assert_eq!(singularize("teeth"), "tooth");
        assert_eq!(singularize("people"), "person");
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

    #[test]
    fn test_pluralize_exceptions() {
        let result = pluralize("child");
        assert_eq!(result, vec!["children"]);

        let result = pluralize("ox");
        assert_eq!(result, vec!["oxen"]);

        let result = pluralize("man");
        assert_eq!(result, vec!["men"]);

        let result = pluralize("woman");
        assert_eq!(result, vec!["women"]);

        let result = pluralize("mouse");
        assert_eq!(result, vec!["mice"]);

        let result = pluralize("goose");
        assert_eq!(result, vec!["geese"]);

        let result = pluralize("tooth");
        assert_eq!(result, vec!["teeth"]);

        let result = pluralize("person");
        assert_eq!(result, vec!["people"]);
    }
}

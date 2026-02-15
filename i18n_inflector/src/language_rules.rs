//! The `LanguageRules` trait standardizes the interface between the core engine and locale modules.

use alloc::borrow::Cow;
use alloc::vec::Vec;

/// Function type for singularization.
pub type SingularizeFn = for<'a> fn(&'a str) -> Cow<'a, str>;

/// Function type for pluralization.
pub type PluralizeFn = for<'a> fn(&'a str) -> Vec<Cow<'a, str>>;

/// A standardized interface for language-specific inflection rules.
///
/// Each language module implements this trait to provide singularization
/// and pluralization with optional irregular/exception handling.
pub trait LanguageRules {
    /// Returns the ISO 639-1 two-letter language code for this language.
    fn language(&self) -> &'static str;

    /// Converts a potentially plural word to its singular form.
    fn singularize<'a>(&self, name: &'a str) -> Cow<'a, str>;

    /// Returns a list of possible plural forms for a word.
    fn pluralize<'a>(&self, name: &'a str) -> Vec<Cow<'a, str>>;
}

/// A concrete implementation of [`LanguageRules`] backed by function pointers.
///
/// This struct can be stored in static maps since all its fields are
/// compile-time constants.
#[derive(Debug)]
pub struct LanguageRuleSet {
    /// The ISO 639-1 two-letter language code.
    pub(crate) language: &'static str,
    /// Function to singularize a word.
    pub(crate) singularize_fn: SingularizeFn,
    /// Function to pluralize a word.
    pub(crate) pluralize_fn: PluralizeFn,
}

impl LanguageRules for LanguageRuleSet {
    fn language(&self) -> &'static str {
        self.language
    }

    fn singularize<'a>(&self, name: &'a str) -> Cow<'a, str> {
        (self.singularize_fn)(name)
    }

    fn pluralize<'a>(&self, name: &'a str) -> Vec<Cow<'a, str>> {
        (self.pluralize_fn)(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use alloc::vec;

    fn test_singularize(name: &str) -> Cow<'_, str> {
        Cow::Borrowed(name)
    }

    fn test_pluralize(name: &str) -> Vec<Cow<'_, str>> {
        vec![Cow::Borrowed(name)]
    }

    fn test_singularize_with_exc(name: &str) -> Cow<'_, str> {
        static EXC: &[(&str, &str)] = &[("children", "child"), ("oxen", "ox")];
        for &(plural, singular) in EXC {
            if name == plural {
                return Cow::Borrowed(singular);
            }
        }
        Cow::Borrowed(name)
    }

    fn test_pluralize_with_exc(name: &str) -> Vec<Cow<'_, str>> {
        static EXC: &[(&str, &str)] = &[("child", "children"), ("ox", "oxen")];
        for &(singular, plural) in EXC {
            if name == singular {
                return vec![Cow::Borrowed(plural)];
            }
        }
        vec![Cow::Borrowed(name)]
    }

    static TEST_RULES: LanguageRuleSet = LanguageRuleSet {
        language: "xx",
        singularize_fn: test_singularize,
        pluralize_fn: test_pluralize,
    };

    static TEST_RULES_WITH_EXCEPTIONS: LanguageRuleSet = LanguageRuleSet {
        language: "xy",
        singularize_fn: test_singularize_with_exc,
        pluralize_fn: test_pluralize_with_exc,
    };

    #[test]
    fn test_language() {
        assert_eq!(TEST_RULES.language(), "xx");
        assert_eq!(TEST_RULES_WITH_EXCEPTIONS.language(), "xy");
    }

    #[test]
    fn test_identity_singularize() {
        assert_eq!(TEST_RULES.singularize("test"), "test");
    }

    #[test]
    fn test_identity_pluralize() {
        assert_eq!(TEST_RULES.pluralize("test"), vec!["test"]);
    }

    #[test]
    fn test_exceptions_singularize() {
        assert_eq!(TEST_RULES_WITH_EXCEPTIONS.singularize("children"), "child");
        assert_eq!(TEST_RULES_WITH_EXCEPTIONS.singularize("oxen"), "ox");
        assert_eq!(TEST_RULES_WITH_EXCEPTIONS.singularize("other"), "other");
    }

    #[test]
    fn test_exceptions_pluralize() {
        assert_eq!(
            TEST_RULES_WITH_EXCEPTIONS.pluralize("child"),
            vec!["children"]
        );
        assert_eq!(TEST_RULES_WITH_EXCEPTIONS.pluralize("ox"), vec!["oxen"]);
        assert_eq!(TEST_RULES_WITH_EXCEPTIONS.pluralize("other"), vec!["other"]);
    }

    #[test]
    fn test_debug() {
        let _ = format!("{TEST_RULES:?}");
    }
}

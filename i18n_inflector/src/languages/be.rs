//! Belarusian (be) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::uk::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "be",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

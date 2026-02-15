//! Haitian Creole (ht) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::ja::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ht",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

//! Sango (sg) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::ja::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "sg",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

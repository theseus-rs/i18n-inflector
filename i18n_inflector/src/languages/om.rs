//! Oromo (om) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::so::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "om",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

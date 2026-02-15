//! Cornish (kw) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::cy::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "kw",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

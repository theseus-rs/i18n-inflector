//! Luxembourgish (lb) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::de::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "lb",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

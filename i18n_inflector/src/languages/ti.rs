//! Tigrinya (ti) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::am::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ti",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

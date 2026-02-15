//! Swati (ss) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::zu::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ss",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

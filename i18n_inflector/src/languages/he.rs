//! Hebrew (he) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::ar::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "he",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

//! Kinyarwanda (rw) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::sw::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "rw",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

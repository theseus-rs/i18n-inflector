//! Interlingua (ia) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::fr::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ia",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

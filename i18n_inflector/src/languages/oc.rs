//! Occitan (oc) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::fr::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "oc",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

//! Galician (gl) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::pt::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "gl",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

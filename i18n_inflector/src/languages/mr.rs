//! Marathi (mr) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::hi::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "mr",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

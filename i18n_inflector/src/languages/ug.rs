//! Uighur (ug) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::tr::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ug",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

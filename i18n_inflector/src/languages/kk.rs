//! Kikuyu (ki) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::tr::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "kk",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

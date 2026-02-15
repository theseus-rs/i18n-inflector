//! Western Frisian (fy) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::nl::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "fy",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

//! Scottish Gaelic (gd) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::ga::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "gd",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

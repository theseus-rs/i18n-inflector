//! Tajik (tg) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::fa::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "tg",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

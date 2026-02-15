//! Faroese (fo) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::is::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "fo",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

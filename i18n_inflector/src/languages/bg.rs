//! Bulgarian (bg) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::hr::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "bg",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

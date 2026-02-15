//! Aragonese (an) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::es::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "an",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

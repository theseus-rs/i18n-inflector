//! Luba-Katanga (lu) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::sw::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "lu",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

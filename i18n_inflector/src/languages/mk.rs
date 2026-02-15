//! Macedonian (mk) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::hr::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "mk",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

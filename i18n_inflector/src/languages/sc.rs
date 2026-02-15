//! Sardinian (sc) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::it::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "sc",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

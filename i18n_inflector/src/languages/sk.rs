//! Slovak (sk) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::cs::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "sk",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

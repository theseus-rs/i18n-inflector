//! Bashkir (ba) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::tr::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ba",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

//! Norwegian Bokmål (nb) inflection rules.

use crate::language_rules::LanguageRuleSet;

pub(crate) use super::da::{pluralize, singularize};

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "nb",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

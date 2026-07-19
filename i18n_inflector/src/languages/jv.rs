use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[LexicalClassSpec::new(
    "explicit-reduplication",
    "explicit plural by full reduplication",
    Rule::Reduplicate("-"),
)];

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::with_alternatives(
    "kebo",
    "kebo",
    &["kebo-kebo"],
)];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("jv", "jv", true, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("jv", &[&super::PROFILE]);
    }
}

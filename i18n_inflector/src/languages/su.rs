use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[LexicalClassSpec::new(
    "explicit-reduplication",
    "explicit plural by full reduplication",
    Rule::Reduplicate("-"),
)];

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::with_alternatives(
    "imah",
    "imah",
    &["imah-imah"],
)];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("su", "su", true, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("su", &[&super::PROFILE]);
    }
}

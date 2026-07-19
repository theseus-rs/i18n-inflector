use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[LexicalClassSpec::new(
    "regular-s",
    "regular nouns taking -s without a stem change",
    Rule::Suffix("s"),
)];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("cairell", "cairells"),
    VerifiedLexeme::new("llautó", "llautons"),
    VerifiedLexeme::new("àgata", "àgates"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ca", "ca", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ca", &[&super::PROFILE]);
    }
}

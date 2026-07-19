use crate::profile::{LanguageProfile, Rule, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("mitã", "mitãkuéra"),
    VerifiedLexeme::new("óga", "ogakuéra"),
    VerifiedLexeme::new("jagua", "jaguakuéra"),
];

pub(crate) static PROFILE: LanguageProfile = LanguageProfile::new(
    "gn",
    "gn",
    false,
    Some(Rule::Suffix("kuéra")),
    &[],
    (false, false),
    LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("gn", &[&super::PROFILE]);
    }
}

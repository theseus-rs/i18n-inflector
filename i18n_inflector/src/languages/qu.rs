use crate::profile::{LanguageProfile, Rule, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("allqu", "allqukuna"),
    VerifiedLexeme::new("wasi", "wasikuna"),
    VerifiedLexeme::new("runa", "runakuna"),
];

pub(crate) static PROFILE: LanguageProfile = LanguageProfile::new(
    "qu",
    "qu",
    false,
    Some(Rule::Suffix("kuna")),
    &[],
    (false, false),
    LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("qu", &[&super::PROFILE]);
    }
}

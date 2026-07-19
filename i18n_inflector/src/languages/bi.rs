use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("man", "man"),
    VerifiedLexeme::new("haos", "haos"),
    VerifiedLexeme::new("buk", "buk"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("bi", "bi", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("bi", &[&super::PROFILE]);
    }
}

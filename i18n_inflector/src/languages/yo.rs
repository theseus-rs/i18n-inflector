use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("ènìyàn", "ènìyàn"),
    VerifiedLexeme::new("ilé", "ilé"),
    VerifiedLexeme::new("ìwé", "ìwé"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("yo", "yo", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("yo", &[&super::PROFILE]);
    }
}

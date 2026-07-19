use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("පොත", "පොත්")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("si", "si", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("si", &[&super::PROFILE]);
    }
}

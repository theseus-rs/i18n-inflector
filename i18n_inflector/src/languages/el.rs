use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("βούτυρο", "βούτυρα")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("el", "el", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("el", &[&super::PROFILE]);
    }
}

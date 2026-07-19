use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("jin", "jin")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ku", "ku-Latn", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ku", &[&super::PROFILE]);
    }
}

use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("mac", "mic")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("gd", "gd", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("gd", &[&super::PROFILE]);
    }
}

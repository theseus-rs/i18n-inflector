use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("رِسَالَة", "رَسَائِلُ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ar", "ar", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ar", &[&super::PROFILE]);
    }
}

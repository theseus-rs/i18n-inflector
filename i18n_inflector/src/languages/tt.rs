use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("кош", "кошлар")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("tt", "tt", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("tt", &[&super::PROFILE]);
    }
}

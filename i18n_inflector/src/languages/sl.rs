use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("hči", "hčere")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sl", "sl", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sl", &[&super::PROFILE]);
    }
}

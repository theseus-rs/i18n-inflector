use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("קאַץ", "קעץ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("yi", "yi-Hebr", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("yi", &[&super::PROFILE]);
    }
}

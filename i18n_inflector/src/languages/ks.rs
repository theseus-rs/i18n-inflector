use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("نال", "نال")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ks", "ks-Arab", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ks", &[&super::PROFILE]);
    }
}

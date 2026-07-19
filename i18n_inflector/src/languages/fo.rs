use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("týsdagur", "týsdagar")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("fo", "fo", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("fo", &[&super::PROFILE]);
    }
}

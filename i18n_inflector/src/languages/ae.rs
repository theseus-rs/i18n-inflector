use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("nar", "narō")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ae", "ae", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ae", &[&super::PROFILE]);
    }
}

use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("motho", "batho")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("tn", "tn", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("tn", &[&super::PROFILE]);
    }
}

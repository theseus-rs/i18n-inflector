use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("moral", "morali")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("hr", "hr", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("hr", &[&super::PROFILE]);
    }
}

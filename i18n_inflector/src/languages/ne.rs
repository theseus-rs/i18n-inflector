use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("किताब", "किताबहरू")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ne", "ne", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ne", &[&super::PROFILE]);
    }
}

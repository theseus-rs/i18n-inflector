use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("nassa", "nassae")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("la", "la", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("la", &[&super::PROFILE]);
    }
}

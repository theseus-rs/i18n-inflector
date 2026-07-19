use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("illu", "illut")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("kl", "kl", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("kl", &[&super::PROFILE]);
    }
}

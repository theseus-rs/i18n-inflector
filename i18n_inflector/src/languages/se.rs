use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("davvisámegiella", "davvisámegielat")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("se", "se", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("se", &[&super::PROFILE]);
    }
}

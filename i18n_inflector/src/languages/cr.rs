use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("atim", "atimwak")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("cr", "cr", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("cr", &[&super::PROFILE]);
    }
}

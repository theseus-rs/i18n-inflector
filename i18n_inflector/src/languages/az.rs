use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ada", "adalar")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("az", "az-Latn", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("az", &[&super::PROFILE]);
    }
}

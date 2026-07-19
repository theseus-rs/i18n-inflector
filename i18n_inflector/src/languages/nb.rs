use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("dør", "dører")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("nb", "nb", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("nb", &[&super::PROFILE]);
    }
}

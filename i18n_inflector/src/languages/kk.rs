use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("жел", "желдер")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("kk", "kk", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("kk", &[&super::PROFILE]);
    }
}

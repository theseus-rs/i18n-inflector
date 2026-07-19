use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ገዛ", "ኣባይቲ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ti", "ti", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ti", &[&super::PROFILE]);
    }
}

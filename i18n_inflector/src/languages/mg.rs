use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("olona", "olona"),
    VerifiedLexeme::new("trano", "trano"),
    VerifiedLexeme::new("boky", "boky"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("mg", "mg", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("mg", &[&super::PROFILE]);
    }
}

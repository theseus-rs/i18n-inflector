use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("nit", "nit"),
    VerifiedLexeme::new("kër", "kër"),
    VerifiedLexeme::new("téere", "téere"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("wo", "wo", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("wo", &[&super::PROFILE]);
    }
}

use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("tau", "tau"),
    VerifiedLexeme::new("ruma", "ruma"),
    VerifiedLexeme::new("sisia", "sisia"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ho", "ho", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ho", &[&super::PROFILE]);
    }
}

use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("zo", "zo"),
    VerifiedLexeme::new("da", "da"),
    VerifiedLexeme::new("mbëtï", "mbëtï"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sg", "sg", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sg", &[&super::PROFILE]);
    }
}

use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("изузетак", "изузеци")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sr", "sr-Cyrl", false, None, &[], (false, false), LEXEMES);

const LATN_LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("izuzetak", "izuzeci")];

pub(crate) static LATN: LanguageProfile = LanguageProfile::new(
    "sr",
    "sr-Latn",
    false,
    None,
    &[],
    (false, false),
    LATN_LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sr", &[&super::PROFILE, &super::LATN]);
    }
}

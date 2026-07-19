use crate::profile::{LanguageProfile, VerifiedLexeme};

const HANS_LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("人", "人"),
    VerifiedLexeme::new("房子", "房子"),
    VerifiedLexeme::new("书", "书"),
];

const HANT_LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("人", "人"),
    VerifiedLexeme::new("房子", "房子"),
    VerifiedLexeme::new("書", "書"),
];

pub(crate) static PROFILE: LanguageProfile = LanguageProfile::new(
    "zh",
    "zh-Hans",
    true,
    None,
    &[],
    (false, false),
    HANS_LEXEMES,
);

pub(crate) static HANT: LanguageProfile = LanguageProfile::new(
    "zh",
    "zh-Hant",
    true,
    None,
    &[],
    (false, false),
    HANT_LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("zh", &[&super::PROFILE, &super::HANT]);
    }
}

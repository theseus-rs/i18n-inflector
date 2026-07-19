use crate::profile::{LanguageProfile, Rule, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("gul", "gullar"),
    VerifiedLexeme::new("daraxt", "daraxtlar"),
    VerifiedLexeme::new("kitob", "kitoblar"),
];

pub(crate) static PROFILE: LanguageProfile = LanguageProfile::new(
    "uz",
    "uz-Latn",
    false,
    Some(Rule::Suffix("lar")),
    &[],
    (false, false),
    LEXEMES,
);

const CYRL_LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("гул", "гуллар"),
    VerifiedLexeme::new("дарахт", "дарахтлар"),
    VerifiedLexeme::new("китоб", "китоблар"),
];

pub(crate) static CYRL: LanguageProfile = LanguageProfile::new(
    "uz",
    "uz-Cyrl",
    false,
    None,
    &[],
    (false, false),
    CYRL_LEXEMES,
);

const ARAB_LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("آت", "آتلر")];

pub(crate) static ARAB: LanguageProfile = LanguageProfile::new(
    "uz",
    "uz-Arab",
    false,
    None,
    &[],
    (false, false),
    ARAB_LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles(
            "uz",
            &[&super::PROFILE, &super::CYRL, &super::ARAB],
        );
    }
}

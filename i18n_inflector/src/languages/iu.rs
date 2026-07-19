use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ᐃᓄᒃ", "ᐃᓄᐃᑦ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("iu", "iu-Cans", false, None, &[], (false, true), LEXEMES);

const LATN_LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("inuk", "inuit")];

pub(crate) static LATN: LanguageProfile = LanguageProfile::new(
    "iu",
    "iu-Latn",
    false,
    None,
    &[],
    (false, true),
    LATN_LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("iu", &[&super::PROFILE, &super::LATN]);
    }
}

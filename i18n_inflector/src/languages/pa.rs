use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ਘੋੜਾ", "ਘੋੜੇ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("pa", "pa-Guru", false, None, &[], (false, false), LEXEMES);

const ARAB_LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("گھوڑا", "گھوڑے")];

pub(crate) static ARAB: LanguageProfile = LanguageProfile::new(
    "pa",
    "pa-Arab",
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
        crate::languages::assert_language_profiles("pa", &[&super::PROFILE, &super::ARAB]);
    }
}

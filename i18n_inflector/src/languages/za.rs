use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("vunz", "vunz"),
    VerifiedLexeme::new("ranz", "ranz"),
    VerifiedLexeme::new("saw", "saw"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("za", "za", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("za", &[&super::PROFILE]);
    }
}

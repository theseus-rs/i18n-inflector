use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ꊿ", "ꊿ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ii", "ii", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ii", &[&super::PROFILE]);
    }
}

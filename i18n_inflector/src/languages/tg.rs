use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("рафиқ", "рафиқон")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("tg", "tg", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("tg", &[&super::PROFILE]);
    }
}

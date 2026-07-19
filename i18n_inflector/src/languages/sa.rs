use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("तारा", "ताराः")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sa", "sa", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sa", &[&super::PROFILE]);
    }
}

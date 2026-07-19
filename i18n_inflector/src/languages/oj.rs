use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("Aamoo", "Aamoog")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("oj", "oj", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("oj", &[&super::PROFILE]);
    }
}

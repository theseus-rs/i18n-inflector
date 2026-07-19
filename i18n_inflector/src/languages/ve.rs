use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("muthu", "vhathu")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ve", "ve", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ve", &[&super::PROFILE]);
    }
}

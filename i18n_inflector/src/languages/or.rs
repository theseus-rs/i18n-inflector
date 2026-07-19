use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ପିଲା", "ପିଲାମାନେ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("or", "or", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("or", &[&super::PROFILE]);
    }
}

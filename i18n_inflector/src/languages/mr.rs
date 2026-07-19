use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("मुलगा", "मुलगे")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("mr", "mr", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("mr", &[&super::PROFILE]);
    }
}

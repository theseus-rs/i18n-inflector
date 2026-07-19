use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ном", "номууд")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("mn", "mn-Cyrl", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("mn", &[&super::PROFILE]);
    }
}

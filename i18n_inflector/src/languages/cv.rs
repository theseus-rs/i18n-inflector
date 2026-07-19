use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ҫурт", "ҫуртсем")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("cv", "cv", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("cv", &[&super::PROFILE]);
    }
}

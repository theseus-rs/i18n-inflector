use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("kath", "kathes")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("kw", "kw", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("kw", &[&super::PROFILE]);
    }
}

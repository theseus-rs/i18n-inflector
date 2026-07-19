use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("beam", "beammen")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("fy", "fy", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("fy", &[&super::PROFILE]);
    }
}

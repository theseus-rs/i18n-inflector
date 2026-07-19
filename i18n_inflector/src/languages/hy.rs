use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("կյանք", "կյանքեր")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("hy", "hy", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("hy", &[&super::PROFILE]);
    }
}

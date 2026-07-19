use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("མི", "མི"),
    VerifiedLexeme::new("ཁྱིམ", "ཁྱིམ"),
    VerifiedLexeme::new("དཔེ་དེབ", "དཔེ་དེབ"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("dz", "dz", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("dz", &[&super::PROFILE]);
    }
}

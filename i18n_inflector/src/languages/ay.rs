use crate::profile::{LanguageProfile, Rule, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("uta", "utanaka"),
    VerifiedLexeme::new("anu", "anunaka"),
    VerifiedLexeme::new("wawa", "wawanaka"),
];

pub(crate) static PROFILE: LanguageProfile = LanguageProfile::new(
    "ay",
    "ay",
    false,
    Some(Rule::Suffix("naka")),
    &[],
    (false, false),
    LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ay", &[&super::PROFILE]);
    }
}

use crate::profile::{LanguageProfile, Rule, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("vol", "vols"),
    VerifiedLexeme::new("dom", "doms"),
    VerifiedLexeme::new("buk", "buks"),
];

pub(crate) static PROFILE: LanguageProfile = LanguageProfile::new(
    "vo",
    "vo",
    false,
    Some(Rule::Suffix("s")),
    &[],
    (false, false),
    LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("vo", &[&super::PROFILE]);
    }
}

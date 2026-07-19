use crate::profile::{LanguageProfile, Rule, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("kato", "katoj"),
    VerifiedLexeme::new("hundo", "hundoj"),
    VerifiedLexeme::new("libro", "libroj"),
];

pub(crate) static PROFILE: LanguageProfile = LanguageProfile::new(
    "eo",
    "eo",
    false,
    Some(Rule::ReplaceSuffix {
        singular: "o",
        plural: "oj",
    }),
    &[],
    (false, false),
    LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("eo", &[&super::PROFILE]);
    }
}

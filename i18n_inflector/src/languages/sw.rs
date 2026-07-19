use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new(
        "m-wa",
        "class 1/2 human nouns",
        Rule::ReplacePrefix {
            singular: "m",
            plural: "wa",
        },
    ),
    LexicalClassSpec::new(
        "ki-vi",
        "class 7/8 nouns",
        Rule::ReplacePrefix {
            singular: "ki",
            plural: "vi",
        },
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("mtu", "watu"),
    VerifiedLexeme::new("mtoto", "watoto"),
    VerifiedLexeme::new("kitabu", "vitabu"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sw", "sw", false, None, CLASSES, (false, true), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sw", &[&super::PROFILE]);
    }
}

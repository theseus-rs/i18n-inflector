use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new(
        "after-vowel-s",
        "nouns ending in a vowel and taking -s",
        Rule::Suffix("s"),
    ),
    LexicalClassSpec::new(
        "after-consonant-es",
        "nouns ending in a consonant other than -c and taking -es",
        Rule::Suffix("es"),
    ),
    LexicalClassSpec::new(
        "final-c-ches",
        "nouns changing final -c to -ches",
        Rule::ReplaceSuffix {
            singular: "c",
            plural: "ches",
        },
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("nation", "nationes"),
    VerifiedLexeme::new("catto", "cattos"),
    VerifiedLexeme::new("can", "canes"),
    VerifiedLexeme::new("urso", "ursos"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ia", "ia", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ia", &[&super::PROFILE]);
    }
}

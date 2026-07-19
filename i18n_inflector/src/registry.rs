use crate::LanguageProfile;
use crate::languages;

pub(crate) static BASE_PROFILES: &[(&str, &LanguageProfile)] = &[
    ("aa", &languages::aa::PROFILE),
    ("ab", &languages::ab::PROFILE),
    ("ae", &languages::ae::PROFILE),
    ("af", &languages::af::PROFILE),
    ("ak", &languages::ak::PROFILE),
    ("am", &languages::am::PROFILE),
    ("an", &languages::an::PROFILE),
    ("ar", &languages::ar::PROFILE),
    ("as", &languages::r#as::PROFILE),
    ("av", &languages::av::PROFILE),
    ("ay", &languages::ay::PROFILE),
    ("az", &languages::az::PROFILE),
    ("ba", &languages::ba::PROFILE),
    ("be", &languages::be::PROFILE),
    ("bg", &languages::bg::PROFILE),
    ("bi", &languages::bi::PROFILE),
    ("bm", &languages::bm::PROFILE),
    ("bn", &languages::bn::PROFILE),
    ("bo", &languages::bo::PROFILE),
    ("br", &languages::br::PROFILE),
    ("bs", &languages::bs::PROFILE),
    ("ca", &languages::ca::PROFILE),
    ("ce", &languages::ce::PROFILE),
    ("ch", &languages::ch::PROFILE),
    ("co", &languages::co::PROFILE),
    ("cr", &languages::cr::PROFILE),
    ("cs", &languages::cs::PROFILE),
    ("cu", &languages::cu::PROFILE),
    ("cv", &languages::cv::PROFILE),
    ("cy", &languages::cy::PROFILE),
    ("da", &languages::da::PROFILE),
    ("de", &languages::de::PROFILE),
    ("dv", &languages::dv::PROFILE),
    ("dz", &languages::dz::PROFILE),
    ("ee", &languages::ee::PROFILE),
    ("el", &languages::el::PROFILE),
    ("en", &languages::en::PROFILE),
    ("eo", &languages::eo::PROFILE),
    ("es", &languages::es::PROFILE),
    ("et", &languages::et::PROFILE),
    ("eu", &languages::eu::PROFILE),
    ("fa", &languages::fa::PROFILE),
    ("ff", &languages::ff::PROFILE),
    ("fi", &languages::fi::PROFILE),
    ("fj", &languages::fj::PROFILE),
    ("fo", &languages::fo::PROFILE),
    ("fr", &languages::fr::PROFILE),
    ("fy", &languages::fy::PROFILE),
    ("ga", &languages::ga::PROFILE),
    ("gd", &languages::gd::PROFILE),
    ("gl", &languages::gl::PROFILE),
    ("gn", &languages::gn::PROFILE),
    ("gu", &languages::gu::PROFILE),
    ("gv", &languages::gv::PROFILE),
    ("ha", &languages::ha::PROFILE),
    ("he", &languages::he::PROFILE),
    ("hi", &languages::hi::PROFILE),
    ("ho", &languages::ho::PROFILE),
    ("hr", &languages::hr::PROFILE),
    ("ht", &languages::ht::PROFILE),
    ("hu", &languages::hu::PROFILE),
    ("hy", &languages::hy::PROFILE),
    ("hz", &languages::hz::PROFILE),
    ("ia", &languages::ia::PROFILE),
    ("id", &languages::id::PROFILE),
    ("ie", &languages::ie::PROFILE),
    ("ig", &languages::ig::PROFILE),
    ("ii", &languages::ii::PROFILE),
    ("ik", &languages::ik::PROFILE),
    ("io", &languages::io::PROFILE),
    ("is", &languages::is::PROFILE),
    ("it", &languages::it::PROFILE),
    ("iu", &languages::iu::PROFILE),
    ("ja", &languages::ja::PROFILE),
    ("jv", &languages::jv::PROFILE),
    ("ka", &languages::ka::PROFILE),
    ("kg", &languages::kg::PROFILE),
    ("ki", &languages::ki::PROFILE),
    ("kj", &languages::kj::PROFILE),
    ("kk", &languages::kk::PROFILE),
    ("kl", &languages::kl::PROFILE),
    ("km", &languages::km::PROFILE),
    ("kn", &languages::kn::PROFILE),
    ("ko", &languages::ko::PROFILE),
    ("kr", &languages::kr::PROFILE),
    ("ks", &languages::ks::PROFILE),
    ("ku", &languages::ku::PROFILE),
    ("kv", &languages::kv::PROFILE),
    ("kw", &languages::kw::PROFILE),
    ("ky", &languages::ky::PROFILE),
    ("la", &languages::la::PROFILE),
    ("lb", &languages::lb::PROFILE),
    ("lg", &languages::lg::PROFILE),
    ("li", &languages::li::PROFILE),
    ("ln", &languages::ln::PROFILE),
    ("lo", &languages::lo::PROFILE),
    ("lt", &languages::lt::PROFILE),
    ("lu", &languages::lu::PROFILE),
    ("lv", &languages::lv::PROFILE),
    ("mg", &languages::mg::PROFILE),
    ("mh", &languages::mh::PROFILE),
    ("mi", &languages::mi::PROFILE),
    ("mk", &languages::mk::PROFILE),
    ("ml", &languages::ml::PROFILE),
    ("mn", &languages::mn::PROFILE),
    ("mr", &languages::mr::PROFILE),
    ("ms", &languages::ms::PROFILE),
    ("mt", &languages::mt::PROFILE),
    ("my", &languages::my::PROFILE),
    ("na", &languages::na::PROFILE),
    ("nb", &languages::nb::PROFILE),
    ("nd", &languages::nd::PROFILE),
    ("ne", &languages::ne::PROFILE),
    ("ng", &languages::ng::PROFILE),
    ("nl", &languages::nl::PROFILE),
    ("nn", &languages::nn::PROFILE),
    ("no", &languages::no::PROFILE),
    ("nr", &languages::nr::PROFILE),
    ("nv", &languages::nv::PROFILE),
    ("ny", &languages::ny::PROFILE),
    ("oc", &languages::oc::PROFILE),
    ("oj", &languages::oj::PROFILE),
    ("om", &languages::om::PROFILE),
    ("or", &languages::or::PROFILE),
    ("os", &languages::os::PROFILE),
    ("pa", &languages::pa::PROFILE),
    ("pi", &languages::pi::PROFILE),
    ("pl", &languages::pl::PROFILE),
    ("ps", &languages::ps::PROFILE),
    ("pt", &languages::pt::PROFILE),
    ("qu", &languages::qu::PROFILE),
    ("rm", &languages::rm::PROFILE),
    ("rn", &languages::rn::PROFILE),
    ("ro", &languages::ro::PROFILE),
    ("ru", &languages::ru::PROFILE),
    ("rw", &languages::rw::PROFILE),
    ("sa", &languages::sa::PROFILE),
    ("sc", &languages::sc::PROFILE),
    ("sd", &languages::sd::PROFILE),
    ("se", &languages::se::PROFILE),
    ("sg", &languages::sg::PROFILE),
    ("si", &languages::si::PROFILE),
    ("sk", &languages::sk::PROFILE),
    ("sl", &languages::sl::PROFILE),
    ("sm", &languages::sm::PROFILE),
    ("sn", &languages::sn::PROFILE),
    ("so", &languages::so::PROFILE),
    ("sq", &languages::sq::PROFILE),
    ("sr", &languages::sr::PROFILE),
    ("ss", &languages::ss::PROFILE),
    ("st", &languages::st::PROFILE),
    ("su", &languages::su::PROFILE),
    ("sv", &languages::sv::PROFILE),
    ("sw", &languages::sw::PROFILE),
    ("ta", &languages::ta::PROFILE),
    ("te", &languages::te::PROFILE),
    ("tg", &languages::tg::PROFILE),
    ("th", &languages::th::PROFILE),
    ("ti", &languages::ti::PROFILE),
    ("tk", &languages::tk::PROFILE),
    ("tl", &languages::tl::PROFILE),
    ("tn", &languages::tn::PROFILE),
    ("to", &languages::to::PROFILE),
    ("tr", &languages::tr::PROFILE),
    ("ts", &languages::ts::PROFILE),
    ("tt", &languages::tt::PROFILE),
    ("tw", &languages::tw::PROFILE),
    ("ty", &languages::ty::PROFILE),
    ("ug", &languages::ug::PROFILE),
    ("uk", &languages::uk::PROFILE),
    ("ur", &languages::ur::PROFILE),
    ("uz", &languages::uz::PROFILE),
    ("ve", &languages::ve::PROFILE),
    ("vi", &languages::vi::PROFILE),
    ("vo", &languages::vo::PROFILE),
    ("wa", &languages::wa::PROFILE),
    ("wo", &languages::wo::PROFILE),
    ("xh", &languages::xh::PROFILE),
    ("yi", &languages::yi::PROFILE),
    ("yo", &languages::yo::PROFILE),
    ("za", &languages::za::PROFILE),
    ("zh", &languages::zh::PROFILE),
    ("zu", &languages::zu::PROFILE),
];

pub(crate) static SCRIPT_PROFILES: &[(&str, &LanguageProfile)] = &[
    ("az-Latn", &languages::az::PROFILE),
    ("be-Cyrl", &languages::be::PROFILE),
    ("iu-Cans", &languages::iu::PROFILE),
    ("iu-Latn", &languages::iu::LATN),
    ("ks-Arab", &languages::ks::PROFILE),
    ("ku-Latn", &languages::ku::PROFILE),
    ("mn-Cyrl", &languages::mn::PROFILE),
    ("pa-Arab", &languages::pa::ARAB),
    ("pa-Guru", &languages::pa::PROFILE),
    ("sd-Arab", &languages::sd::PROFILE),
    ("sr-Cyrl", &languages::sr::PROFILE),
    ("sr-Latn", &languages::sr::LATN),
    ("ug-Arab", &languages::ug::PROFILE),
    ("ur-Arab", &languages::ur::PROFILE),
    ("uz-Arab", &languages::uz::ARAB),
    ("uz-Cyrl", &languages::uz::CYRL),
    ("uz-Latn", &languages::uz::PROFILE),
    ("yi-Hebr", &languages::yi::PROFILE),
    ("zh-Hans", &languages::zh::PROFILE),
    ("zh-Hant", &languages::zh::HANT),
];

pub(crate) fn base_profile(language: &str) -> Option<&'static LanguageProfile> {
    BASE_PROFILES
        .iter()
        .find_map(|(code, profile)| (*code == language).then_some(*profile))
}

pub(crate) fn script_profile(locale: &str) -> Option<&'static LanguageProfile> {
    SCRIPT_PROFILES
        .iter()
        .find_map(|(candidate, profile)| (*candidate == locale).then_some(*profile))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, InflectionRequest, Number};
    use alloc::string::ToString;
    use alloc::vec::Vec;

    const ISO_639_1_CODES: &str = "aa ab ae af ak am an ar as av ay az ba be bg bi bm bn bo br bs ca ce ch co cr cs cu cv cy da de dv dz ee el en eo es et eu fa ff fi fj fo fr fy ga gd gl gn gu gv ha he hi ho hr ht hu hy hz ia id ie ig ii ik io is it iu ja jv ka kg ki kj kk kl km kn ko kr ks ku kv kw ky la lb lg li ln lo lt lu lv mg mh mi mk ml mn mr ms mt my na nb nd ne ng nl nn no nr nv ny oc oj om or os pa pi pl ps pt qu rm rn ro ru rw sa sc sd se sg si sk sl sm sn so sq sr ss st su sv sw ta te tg th ti tk tl tn to tr ts tt tw ty ug uk ur uz ve vi vo wa wo xh yi yo za zh zu";

    fn profiles() -> impl Iterator<Item = &'static LanguageProfile> {
        BASE_PROFILES
            .iter()
            .chain(SCRIPT_PROFILES)
            .map(|(_, profile)| *profile)
    }

    #[test]
    fn contains_every_iso_set_one_code() {
        assert_eq!(BASE_PROFILES.len(), 183);
        for code in ISO_639_1_CODES.split_ascii_whitespace() {
            assert!(
                BASE_PROFILES
                    .iter()
                    .any(|(candidate, _)| *candidate == code),
                "missing ISO 639-1 code: {code}"
            );
        }
        for (code, profile) in BASE_PROFILES {
            assert_eq!(*code, profile.language());
            assert_eq!(
                base_profile(code).map(LanguageProfile::language),
                Some(*code)
            );
        }
        assert!(base_profile("xx").is_none());
    }

    #[test]
    fn script_profiles_match_their_keys() {
        assert_eq!(SCRIPT_PROFILES.len(), 20);
        for (locale, profile) in SCRIPT_PROFILES {
            assert_eq!(*locale, profile.locale());
            assert_eq!(
                script_profile(locale).map(LanguageProfile::locale),
                Some(*locale)
            );
        }
        assert!(script_profile("en-Cyrl").is_none());
    }

    #[test]
    fn every_verified_lexeme_round_trips_through_its_profile() {
        for profile in profiles() {
            for entry in profile.lexemes() {
                assert_eq!(
                    profile
                        .inflect(InflectionRequest::singular(entry.lemma()))
                        .map(|forms| forms.primary().to_string()),
                    Ok(entry.lemma().to_string())
                );
                match entry.plural() {
                    Some(plural) => {
                        let forms = profile
                            .inflect(InflectionRequest::plural(entry.lemma()))
                            .unwrap();
                        assert_eq!(forms.primary(), plural);
                        assert_eq!(
                            forms
                                .alternatives()
                                .iter()
                                .map(AsRef::as_ref)
                                .collect::<Vec<_>>(),
                            entry.alternatives()
                        );
                    }
                    None => assert_eq!(
                        profile.inflect(InflectionRequest::plural(entry.lemma())),
                        Err(Error::NoForm {
                            locale: profile.locale(),
                            lemma: entry.lemma().to_string(),
                            number: Number::Plural,
                        })
                    ),
                }
            }
        }
    }

    #[test]
    fn every_profile_has_a_defined_unknown_lemma_policy() {
        for profile in profiles() {
            let result = profile.inflect(InflectionRequest::plural("__unknown_lemma__"));
            if profile.invariant {
                assert!(result.is_ok());
            } else if profile.default_rule.is_some() {
                if profile.locale() == "eo" {
                    assert!(result.is_err());
                } else {
                    assert!(result.is_ok());
                }
            } else {
                assert!(result.is_err());
            }
            if !profile.invariant
                && profile.default_rule.is_none()
                && profile.capabilities().lexical_classes().is_empty()
            {
                assert!(!profile.lexemes().is_empty());
            }
        }
    }
}

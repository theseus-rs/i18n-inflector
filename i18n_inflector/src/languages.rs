//! Language-specific profiles and conformance lexemes.

pub(crate) mod aa;
pub(crate) mod ab;
pub(crate) mod ae;
pub(crate) mod af;
pub(crate) mod ak;
pub(crate) mod am;
pub(crate) mod an;
pub(crate) mod ar;
pub(crate) mod r#as;
pub(crate) mod av;
pub(crate) mod ay;
pub(crate) mod az;
pub(crate) mod ba;
pub(crate) mod be;
pub(crate) mod bg;
pub(crate) mod bi;
pub(crate) mod bm;
pub(crate) mod bn;
pub(crate) mod bo;
pub(crate) mod br;
pub(crate) mod bs;
pub(crate) mod ca;
pub(crate) mod ce;
pub(crate) mod ch;
pub(crate) mod co;
pub(crate) mod cr;
pub(crate) mod cs;
pub(crate) mod cu;
pub(crate) mod cv;
pub(crate) mod cy;
pub(crate) mod da;
pub(crate) mod de;
pub(crate) mod dv;
pub(crate) mod dz;
pub(crate) mod ee;
pub(crate) mod el;
pub(crate) mod en;
pub(crate) mod eo;
pub(crate) mod es;
pub(crate) mod et;
pub(crate) mod eu;
pub(crate) mod fa;
pub(crate) mod ff;
pub(crate) mod fi;
pub(crate) mod fj;
pub(crate) mod fo;
pub(crate) mod fr;
pub(crate) mod fy;
pub(crate) mod ga;
pub(crate) mod gd;
pub(crate) mod gl;
pub(crate) mod gn;
pub(crate) mod gu;
pub(crate) mod gv;
pub(crate) mod ha;
pub(crate) mod he;
pub(crate) mod hi;
pub(crate) mod ho;
pub(crate) mod hr;
pub(crate) mod ht;
pub(crate) mod hu;
pub(crate) mod hy;
pub(crate) mod hz;
pub(crate) mod ia;
pub(crate) mod id;
pub(crate) mod ie;
pub(crate) mod ig;
pub(crate) mod ii;
pub(crate) mod ik;
pub(crate) mod io;
pub(crate) mod is;
pub(crate) mod it;
pub(crate) mod iu;
pub(crate) mod ja;
pub(crate) mod jv;
pub(crate) mod ka;
pub(crate) mod kg;
pub(crate) mod ki;
pub(crate) mod kj;
pub(crate) mod kk;
pub(crate) mod kl;
pub(crate) mod km;
pub(crate) mod kn;
pub(crate) mod ko;
pub(crate) mod kr;
pub(crate) mod ks;
pub(crate) mod ku;
pub(crate) mod kv;
pub(crate) mod kw;
pub(crate) mod ky;
pub(crate) mod la;
pub(crate) mod lb;
pub(crate) mod lg;
pub(crate) mod li;
pub(crate) mod ln;
pub(crate) mod lo;
pub(crate) mod lt;
pub(crate) mod lu;
pub(crate) mod lv;
pub(crate) mod mg;
pub(crate) mod mh;
pub(crate) mod mi;
pub(crate) mod mk;
pub(crate) mod ml;
pub(crate) mod mn;
pub(crate) mod mr;
pub(crate) mod ms;
pub(crate) mod mt;
pub(crate) mod my;
pub(crate) mod na;
pub(crate) mod nb;
pub(crate) mod nd;
pub(crate) mod ne;
pub(crate) mod ng;
pub(crate) mod nl;
pub(crate) mod nn;
pub(crate) mod no;
pub(crate) mod nr;
pub(crate) mod nv;
pub(crate) mod ny;
pub(crate) mod oc;
pub(crate) mod oj;
pub(crate) mod om;
pub(crate) mod or;
pub(crate) mod os;
pub(crate) mod pa;
pub(crate) mod pi;
pub(crate) mod pl;
pub(crate) mod ps;
pub(crate) mod pt;
pub(crate) mod qu;
pub(crate) mod rm;
pub(crate) mod rn;
pub(crate) mod ro;
pub(crate) mod ru;
pub(crate) mod rw;
pub(crate) mod sa;
pub(crate) mod sc;
pub(crate) mod sd;
pub(crate) mod se;
pub(crate) mod sg;
pub(crate) mod si;
pub(crate) mod sk;
pub(crate) mod sl;
pub(crate) mod sm;
pub(crate) mod sn;
pub(crate) mod so;
pub(crate) mod sq;
pub(crate) mod sr;
pub(crate) mod ss;
pub(crate) mod st;
pub(crate) mod su;
pub(crate) mod sv;
pub(crate) mod sw;
pub(crate) mod ta;
pub(crate) mod te;
pub(crate) mod tg;
pub(crate) mod th;
pub(crate) mod ti;
pub(crate) mod tk;
pub(crate) mod tl;
pub(crate) mod tn;
pub(crate) mod to;
pub(crate) mod tr;
pub(crate) mod ts;
pub(crate) mod tt;
pub(crate) mod tw;
pub(crate) mod ty;
pub(crate) mod ug;
pub(crate) mod uk;
pub(crate) mod ur;
pub(crate) mod uz;
pub(crate) mod ve;
pub(crate) mod vi;
pub(crate) mod vo;
pub(crate) mod wa;
pub(crate) mod wo;
pub(crate) mod xh;
pub(crate) mod yi;
pub(crate) mod yo;
pub(crate) mod za;
pub(crate) mod zh;
pub(crate) mod zu;

#[cfg(test)]
fn rule_example(rule: crate::profile::Rule) -> (alloc::string::String, alloc::string::String) {
    use crate::profile::Rule;
    use alloc::format;
    use alloc::string::ToString;

    match rule {
        Rule::Suffix(suffix) => ("unit".to_string(), format!("unit{suffix}")),
        Rule::ReplaceSuffix { singular, plural } => {
            (format!("unit{singular}"), format!("unit{plural}"))
        }
        Rule::ReplacePrefix { singular, plural } => {
            (format!("{singular}unit"), format!("{plural}unit"))
        }
        Rule::Reduplicate(separator) => ("unit".to_string(), format!("unit{separator}unit")),
        Rule::Turkish => ("kitap".to_string(), "kitaplar".to_string()),
    }
}

#[cfg(test)]
fn request_with_class<'a>(
    lemma: &'a str,
    class: Option<&'static str>,
) -> crate::InflectionRequest<'a> {
    use crate::{InflectionRequest, LexicalClassId};

    class.map_or(InflectionRequest::plural(lemma), |class| {
        InflectionRequest::plural(lemma).lexical_class(LexicalClassId::new(class))
    })
}

#[cfg(test)]
fn assert_rule(
    profile: &crate::profile::LanguageProfile,
    class: Option<&'static str>,
    rule: crate::profile::Rule,
) {
    use crate::profile::Rule;
    use alloc::string::ToString;

    let (lemma, expected) = rule_example(rule);
    let request = request_with_class(&lemma, class);
    assert_eq!(
        profile
            .inflect(request)
            .map(|forms| forms.primary().to_string()),
        Ok(expected)
    );
    if rule == Rule::Turkish {
        let request = request_with_class("şehir-test", class);
        assert_eq!(
            profile
                .inflect(request)
                .map(|forms| forms.primary().to_string()),
            Ok("şehir-testler".to_string())
        );
    }
}

#[cfg(test)]
fn assert_language_profiles(
    expected_language: &str,
    profiles: &[&crate::profile::LanguageProfile],
) {
    use crate::{Error, InflectionRequest, Number};
    use alloc::string::ToString;

    assert!(!profiles.is_empty());
    for profile in profiles {
        assert_eq!(profile.language(), expected_language);
        assert!(
            profile.locale() == expected_language
                || profile
                    .locale()
                    .strip_prefix(expected_language)
                    .is_some_and(|suffix| suffix.starts_with('-'))
        );
        assert_eq!(
            profile
                .inflect(InflectionRequest::singular("module-unit"))
                .map(|forms| forms.primary().to_string()),
            Ok("module-unit".to_string())
        );

        assert!(!profile.lexemes().is_empty());
        for lexeme in profile.lexemes() {
            let result = profile.inflect(InflectionRequest::plural(lexeme.lemma()));
            if let Some(plural) = lexeme.plural() {
                assert_eq!(
                    result.as_ref().map(crate::InflectedForms::primary),
                    Ok(plural)
                );
                assert!(result.as_ref().is_ok_and(|forms| {
                    forms
                        .alternatives()
                        .iter()
                        .map(AsRef::as_ref)
                        .eq(lexeme.alternatives().iter().copied())
                }));
            } else {
                assert_eq!(
                    result,
                    Err(Error::NoForm {
                        locale: profile.locale(),
                        lemma: lexeme.lemma().to_string(),
                        number: Number::Plural,
                    })
                );
            }
        }

        if profile.invariant {
            assert_eq!(
                profile
                    .inflect(InflectionRequest::plural("module-unit"))
                    .map(|forms| forms.primary().to_string()),
                Ok("module-unit".to_string())
            );
        }
        if let Some(rule) = profile.default_rule {
            assert_rule(profile, None, rule);
        }
        for class in profile.capabilities().lexical_classes() {
            assert!(!class.id().is_empty());
            assert!(!class.description().is_empty());
            assert_rule(profile, Some(class.id()), class.rule);
        }
    }
}

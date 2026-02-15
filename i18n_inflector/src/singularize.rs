//! Singularization of nouns.

use alloc::borrow::Cow;
use alloc::format;

use crate::error::{Error, Result};
use crate::languages;

/// Converts a potentially plural word to its singular form based on the given locale.
///
/// The locale must be a supported ISO 639 two-letter language code (e.g., `"en"`, `"es"`, `"de"`).
/// Returns an error if the locale is not recognized.
///
/// # Errors
///
/// Returns [`Error`] if the locale is not a supported language code.
///
/// # Examples
///
/// ```
/// use i18n_inflector::singularize;
///
/// // English
/// assert_eq!(singularize("en", "users").unwrap(), "user");
/// assert_eq!(singularize("en", "categories").unwrap(), "category");
/// assert_eq!(singularize("en", "boxes").unwrap(), "box");
///
/// // Spanish
/// assert_eq!(singularize("es", "usuarios").unwrap(), "usuario");
///
/// // German
/// assert_eq!(singularize("de", "produkte").unwrap(), "produkt");
///
/// // Japanese (invariant)
/// assert_eq!(singularize("ja", "user").unwrap(), "user");
///
/// // Unsupported locale returns an error
/// assert!(singularize("xx", "users").is_err());
/// ```
#[expect(clippy::too_many_lines)]
pub fn singularize<'a>(locale: &str, name: &'a str) -> Result<Cow<'a, str>> {
    match locale {
        "af" => Ok(languages::af::singularize(name)),
        "am" => Ok(languages::am::singularize(name)),
        "an" => Ok(languages::an::singularize(name)),
        "ar" => Ok(languages::ar::singularize(name)),
        "as" => Ok(languages::r#as::singularize(name)),
        "az" => Ok(languages::az::singularize(name)),
        "be" => Ok(languages::be::singularize(name)),
        "bg" => Ok(languages::bg::singularize(name)),
        "bn" => Ok(languages::bn::singularize(name)),
        "bo" => Ok(languages::bo::singularize(name)),
        "br" => Ok(languages::br::singularize(name)),
        "bs" => Ok(languages::bs::singularize(name)),
        "ca" => Ok(languages::ca::singularize(name)),
        "co" => Ok(languages::co::singularize(name)),
        "cs" => Ok(languages::cs::singularize(name)),
        "cy" => Ok(languages::cy::singularize(name)),
        "da" => Ok(languages::da::singularize(name)),
        "de" => Ok(languages::de::singularize(name)),
        "el" => Ok(languages::el::singularize(name)),
        "en" => Ok(languages::en::singularize(name)),
        "eo" => Ok(languages::eo::singularize(name)),
        "es" => Ok(languages::es::singularize(name)),
        "et" => Ok(languages::et::singularize(name)),
        "eu" => Ok(languages::eu::singularize(name)),
        "fa" => Ok(languages::fa::singularize(name)),
        "fi" => Ok(languages::fi::singularize(name)),
        "fo" => Ok(languages::fo::singularize(name)),
        "fr" => Ok(languages::fr::singularize(name)),
        "fy" => Ok(languages::fy::singularize(name)),
        "ga" => Ok(languages::ga::singularize(name)),
        "gd" => Ok(languages::gd::singularize(name)),
        "gl" => Ok(languages::gl::singularize(name)),
        "gu" => Ok(languages::gu::singularize(name)),
        "gv" => Ok(languages::gv::singularize(name)),
        "ha" => Ok(languages::ha::singularize(name)),
        "he" => Ok(languages::he::singularize(name)),
        "hi" => Ok(languages::hi::singularize(name)),
        "hr" => Ok(languages::hr::singularize(name)),
        "ht" => Ok(languages::ht::singularize(name)),
        "hu" => Ok(languages::hu::singularize(name)),
        "hy" => Ok(languages::hy::singularize(name)),
        "id" => Ok(languages::id::singularize(name)),
        "ig" => Ok(languages::ig::singularize(name)),
        "is" => Ok(languages::is::singularize(name)),
        "it" => Ok(languages::it::singularize(name)),
        "ja" => Ok(languages::ja::singularize(name)),
        "jv" => Ok(languages::jv::singularize(name)),
        "ka" => Ok(languages::ka::singularize(name)),
        "kk" => Ok(languages::kk::singularize(name)),
        "km" => Ok(languages::km::singularize(name)),
        "kn" => Ok(languages::kn::singularize(name)),
        "ko" => Ok(languages::ko::singularize(name)),
        "ku" => Ok(languages::ku::singularize(name)),
        "kw" => Ok(languages::kw::singularize(name)),
        "ky" => Ok(languages::ky::singularize(name)),
        "la" => Ok(languages::la::singularize(name)),
        "lb" => Ok(languages::lb::singularize(name)),
        "lo" => Ok(languages::lo::singularize(name)),
        "lt" => Ok(languages::lt::singularize(name)),
        "lv" => Ok(languages::lv::singularize(name)),
        "mg" => Ok(languages::mg::singularize(name)),
        "mi" => Ok(languages::mi::singularize(name)),
        "mk" => Ok(languages::mk::singularize(name)),
        "ml" => Ok(languages::ml::singularize(name)),
        "mn" => Ok(languages::mn::singularize(name)),
        "mr" => Ok(languages::mr::singularize(name)),
        "ms" => Ok(languages::ms::singularize(name)),
        "mt" => Ok(languages::mt::singularize(name)),
        "my" => Ok(languages::my::singularize(name)),
        "nb" => Ok(languages::nb::singularize(name)),
        "nd" => Ok(languages::nd::singularize(name)),
        "ne" => Ok(languages::ne::singularize(name)),
        "nl" => Ok(languages::nl::singularize(name)),
        "nn" => Ok(languages::nn::singularize(name)),
        "no" => Ok(languages::no::singularize(name)),
        "nr" => Ok(languages::nr::singularize(name)),
        "oc" => Ok(languages::oc::singularize(name)),
        "or" => Ok(languages::or::singularize(name)),
        "pa" => Ok(languages::pa::singularize(name)),
        "pl" => Ok(languages::pl::singularize(name)),
        "ps" => Ok(languages::ps::singularize(name)),
        "pt" => Ok(languages::pt::singularize(name)),
        "qu" => Ok(languages::qu::singularize(name)),
        "rm" => Ok(languages::rm::singularize(name)),
        "ro" => Ok(languages::ro::singularize(name)),
        "ru" => Ok(languages::ru::singularize(name)),
        "rw" => Ok(languages::rw::singularize(name)),
        "sc" => Ok(languages::sc::singularize(name)),
        "sd" => Ok(languages::sd::singularize(name)),
        "si" => Ok(languages::si::singularize(name)),
        "sk" => Ok(languages::sk::singularize(name)),
        "sl" => Ok(languages::sl::singularize(name)),
        "sm" => Ok(languages::sm::singularize(name)),
        "sn" => Ok(languages::sn::singularize(name)),
        "so" => Ok(languages::so::singularize(name)),
        "sq" => Ok(languages::sq::singularize(name)),
        "sr" => Ok(languages::sr::singularize(name)),
        "ss" => Ok(languages::ss::singularize(name)),
        "st" => Ok(languages::st::singularize(name)),
        "su" => Ok(languages::su::singularize(name)),
        "sv" => Ok(languages::sv::singularize(name)),
        "sw" => Ok(languages::sw::singularize(name)),
        "ta" => Ok(languages::ta::singularize(name)),
        "te" => Ok(languages::te::singularize(name)),
        "tg" => Ok(languages::tg::singularize(name)),
        "th" => Ok(languages::th::singularize(name)),
        "ti" => Ok(languages::ti::singularize(name)),
        "tk" => Ok(languages::tk::singularize(name)),
        "tl" => Ok(languages::tl::singularize(name)),
        "tn" => Ok(languages::tn::singularize(name)),
        "tr" => Ok(languages::tr::singularize(name)),
        "ts" => Ok(languages::ts::singularize(name)),
        "tt" => Ok(languages::tt::singularize(name)),
        "ug" => Ok(languages::ug::singularize(name)),
        "uk" => Ok(languages::uk::singularize(name)),
        "ur" => Ok(languages::ur::singularize(name)),
        "uz" => Ok(languages::uz::singularize(name)),
        "ve" => Ok(languages::ve::singularize(name)),
        "vi" => Ok(languages::vi::singularize(name)),
        "wo" => Ok(languages::wo::singularize(name)),
        "xh" => Ok(languages::xh::singularize(name)),
        "yi" => Ok(languages::yi::singularize(name)),
        "yo" => Ok(languages::yo::singularize(name)),
        "zh" => Ok(languages::zh::singularize(name)),
        "zu" => Ok(languages::zu::singularize(name)),
        _ => Err(Error::new(format!("unsupported locale: {locale}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_af() {
        assert_eq!(singularize("af", "honde").unwrap(), "hond");
    }

    #[test]
    fn test_am() {
        assert_eq!(singularize("am", "betoch").unwrap(), "bet");
    }

    #[test]
    fn test_an() {
        assert_eq!(singularize("an", "usuarios").unwrap(), "usuario");
    }

    #[test]
    fn test_ar() {
        assert_eq!(singularize("ar", "mustahdimin").unwrap(), "mustahdim");
    }

    #[test]
    fn test_as() {
        assert_eq!(singularize("as", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_az() {
        assert_eq!(singularize("az", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_be() {
        assert_eq!(singularize("be", "pradukty").unwrap(), "pradukta");
    }

    #[test]
    fn test_bg() {
        assert_eq!(singularize("bg", "produkti").unwrap(), "produkt");
    }

    #[test]
    fn test_bn() {
        assert_eq!(singularize("bn", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_bo() {
        assert_eq!(singularize("bo", "user").unwrap(), "user");
    }

    #[test]
    fn test_br() {
        assert_eq!(singularize("br", "bagou").unwrap(), "bag");
    }

    #[test]
    fn test_bs() {
        assert_eq!(singularize("bs", "korisnici").unwrap(), "korisnik");
    }

    #[test]
    fn test_ca() {
        assert_eq!(singularize("ca", "gats").unwrap(), "gat");
    }

    #[test]
    fn test_co() {
        assert_eq!(singularize("co", "prodotti").unwrap(), "prodotto");
    }

    #[test]
    fn test_cs() {
        assert_eq!(singularize("cs", "produkty").unwrap(), "produkt");
    }

    #[test]
    fn test_cy() {
        assert_eq!(singularize("cy", "cathod").unwrap(), "cath");
    }

    #[test]
    fn test_da() {
        assert_eq!(singularize("da", "brugere").unwrap(), "brug");
    }

    #[test]
    fn test_de() {
        assert_eq!(singularize("de", "produkte").unwrap(), "produkt");
    }

    #[test]
    fn test_el() {
        assert_eq!(singularize("el", "xristes").unwrap(), "xristis");
    }

    #[test]
    fn test_en() {
        assert_eq!(singularize("en", "users").unwrap(), "user");
    }

    #[test]
    fn test_eo() {
        assert_eq!(singularize("eo", "katoj").unwrap(), "kato");
    }

    #[test]
    fn test_es() {
        assert_eq!(singularize("es", "usuarios").unwrap(), "usuario");
    }

    #[test]
    fn test_et() {
        assert_eq!(singularize("et", "kasutajad").unwrap(), "kasutaja");
    }

    #[test]
    fn test_eu() {
        assert_eq!(singularize("eu", "katuak").unwrap(), "katu");
    }

    #[test]
    fn test_fa() {
        assert_eq!(singularize("fa", "ketabha").unwrap(), "ketab");
    }

    #[test]
    fn test_fi() {
        assert_eq!(singularize("fi", "tuotteet").unwrap(), "tuottee");
    }

    #[test]
    fn test_fo() {
        assert_eq!(singularize("fo", "notendur").unwrap(), "notend");
    }

    #[test]
    fn test_fr() {
        assert_eq!(singularize("fr", "utilisateurs").unwrap(), "utilisateur");
    }

    #[test]
    fn test_fy() {
        assert_eq!(singularize("fy", "klanten").unwrap(), "klant");
    }

    #[test]
    fn test_ga() {
        assert_eq!(singularize("ga", "usaideoiri").unwrap(), "usaideoir");
    }

    #[test]
    fn test_gd() {
        assert_eq!(singularize("gd", "usaideoiri").unwrap(), "usaideoir");
    }

    #[test]
    fn test_gl() {
        assert_eq!(singularize("gl", "produtos").unwrap(), "produto");
    }

    #[test]
    fn test_gu() {
        assert_eq!(singularize("gu", "chokrao").unwrap(), "chokra");
    }

    #[test]
    fn test_gv() {
        assert_eq!(singularize("gv", "usaideoiri").unwrap(), "usaideoir");
    }

    #[test]
    fn test_ha() {
        assert_eq!(singularize("ha", "litattafai").unwrap(), "litattaf");
    }

    #[test]
    fn test_he() {
        assert_eq!(singularize("he", "meshtatfim").unwrap(), "meshtatf");
    }

    #[test]
    fn test_hi() {
        assert_eq!(singularize("hi", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_hr() {
        assert_eq!(singularize("hr", "korisnici").unwrap(), "korisnik");
    }

    #[test]
    fn test_ht() {
        assert_eq!(singularize("ht", "user").unwrap(), "user");
    }

    #[test]
    fn test_hu() {
        assert_eq!(singularize("hu", "felhasznalok").unwrap(), "felhasznalo");
    }

    #[test]
    fn test_hy() {
        assert_eq!(singularize("hy", "girqer").unwrap(), "girq");
    }

    #[test]
    fn test_id() {
        assert_eq!(singularize("id", "user").unwrap(), "user");
    }

    #[test]
    fn test_ig() {
        assert_eq!(singularize("ig", "user").unwrap(), "user");
    }

    #[test]
    fn test_is() {
        assert_eq!(singularize("is", "notendur").unwrap(), "notend");
    }

    #[test]
    fn test_it() {
        assert_eq!(singularize("it", "prodotti").unwrap(), "prodotto");
    }

    #[test]
    fn test_ja() {
        assert_eq!(singularize("ja", "user").unwrap(), "user");
    }

    #[test]
    fn test_jv() {
        assert_eq!(singularize("jv", "user").unwrap(), "user");
    }

    #[test]
    fn test_ka() {
        assert_eq!(singularize("ka", "user").unwrap(), "user");
    }

    #[test]
    fn test_kk() {
        assert_eq!(singularize("kk", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_km() {
        assert_eq!(singularize("km", "user").unwrap(), "user");
    }

    #[test]
    fn test_kn() {
        assert_eq!(singularize("kn", "pustakagalu").unwrap(), "pustaka");
    }

    #[test]
    fn test_ko() {
        assert_eq!(singularize("ko", "user").unwrap(), "user");
    }

    #[test]
    fn test_ku() {
        assert_eq!(singularize("ku", "daristan").unwrap(), "darist");
    }

    #[test]
    fn test_kw() {
        assert_eq!(singularize("kw", "cathod").unwrap(), "cath");
    }

    #[test]
    fn test_ky() {
        assert_eq!(singularize("ky", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_la() {
        assert_eq!(singularize("la", "domini").unwrap(), "dominus");
    }

    #[test]
    fn test_lb() {
        assert_eq!(singularize("lb", "produkte").unwrap(), "produkt");
    }

    #[test]
    fn test_lo() {
        assert_eq!(singularize("lo", "user").unwrap(), "user");
    }

    #[test]
    fn test_lt() {
        assert_eq!(singularize("lt", "vartotojai").unwrap(), "vartotojas");
    }

    #[test]
    fn test_lv() {
        assert_eq!(singularize("lv", "lietotaji").unwrap(), "lietotajs");
    }

    #[test]
    fn test_mg() {
        assert_eq!(singularize("mg", "user").unwrap(), "user");
    }

    #[test]
    fn test_mi() {
        assert_eq!(singularize("mi", "user").unwrap(), "user");
    }

    #[test]
    fn test_mk() {
        assert_eq!(singularize("mk", "korisnici").unwrap(), "korisnik");
    }

    #[test]
    fn test_ml() {
        assert_eq!(singularize("ml", "pustakakal").unwrap(), "pustaka");
    }

    #[test]
    fn test_mn() {
        assert_eq!(singularize("mn", "nomuud").unwrap(), "nom");
    }

    #[test]
    fn test_mr() {
        assert_eq!(singularize("mr", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_ms() {
        assert_eq!(singularize("ms", "user").unwrap(), "user");
    }

    #[test]
    fn test_mt() {
        assert_eq!(singularize("mt", "utenti").unwrap(), "utent");
    }

    #[test]
    fn test_my() {
        assert_eq!(singularize("my", "user").unwrap(), "user");
    }

    #[test]
    fn test_nb() {
        assert_eq!(singularize("nb", "brukere").unwrap(), "bruk");
    }

    #[test]
    fn test_nd() {
        assert_eq!(singularize("nd", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_ne() {
        assert_eq!(singularize("ne", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_nl() {
        assert_eq!(singularize("nl", "klanten").unwrap(), "klant");
    }

    #[test]
    fn test_nn() {
        assert_eq!(singularize("nn", "brukere").unwrap(), "bruk");
    }

    #[test]
    fn test_no() {
        assert_eq!(singularize("no", "brukere").unwrap(), "bruk");
    }

    #[test]
    fn test_nr() {
        assert_eq!(singularize("nr", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_oc() {
        assert_eq!(singularize("oc", "utilisateurs").unwrap(), "utilisateur");
    }

    #[test]
    fn test_or() {
        assert_eq!(singularize("or", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_pa() {
        assert_eq!(singularize("pa", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_pl() {
        assert_eq!(singularize("pl", "produkty").unwrap(), "produkt");
    }

    #[test]
    fn test_ps() {
        assert_eq!(singularize("ps", "kitabuna").unwrap(), "kitab");
    }

    #[test]
    fn test_pt() {
        assert_eq!(singularize("pt", "clientes").unwrap(), "cliente");
    }

    #[test]
    fn test_qu() {
        assert_eq!(singularize("qu", "wasikuna").unwrap(), "wasi");
    }

    #[test]
    fn test_rm() {
        assert_eq!(singularize("rm", "utilisateurs").unwrap(), "utilisateur");
    }

    #[test]
    fn test_ro() {
        assert_eq!(singularize("ro", "utilizatori").unwrap(), "utilizator");
    }

    #[test]
    fn test_ru() {
        assert_eq!(singularize("ru", "klienti").unwrap(), "klient");
    }

    #[test]
    fn test_rw() {
        assert_eq!(singularize("rw", "vitabuni").unwrap(), "vitabu");
    }

    #[test]
    fn test_sc() {
        assert_eq!(singularize("sc", "prodotti").unwrap(), "prodotto");
    }

    #[test]
    fn test_sd() {
        assert_eq!(singularize("sd", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_si() {
        assert_eq!(singularize("si", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_sk() {
        assert_eq!(singularize("sk", "produkty").unwrap(), "produkt");
    }

    #[test]
    fn test_sl() {
        assert_eq!(singularize("sl", "korisnici").unwrap(), "korisnik");
    }

    #[test]
    fn test_sm() {
        assert_eq!(singularize("sm", "user").unwrap(), "user");
    }

    #[test]
    fn test_sn() {
        assert_eq!(singularize("sn", "vitabuni").unwrap(), "vitabu");
    }

    #[test]
    fn test_so() {
        assert_eq!(singularize("so", "buugaagyo").unwrap(), "buugaag");
    }

    #[test]
    fn test_sq() {
        assert_eq!(singularize("sq", "perdoruese").unwrap(), "perdorues");
    }

    #[test]
    fn test_sr() {
        assert_eq!(singularize("sr", "korisnici").unwrap(), "korisnik");
    }

    #[test]
    fn test_ss() {
        assert_eq!(singularize("ss", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_st() {
        assert_eq!(singularize("st", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_su() {
        assert_eq!(singularize("su", "user").unwrap(), "user");
    }

    #[test]
    fn test_sv() {
        assert_eq!(singularize("sv", "produkter").unwrap(), "produkt");
    }

    #[test]
    fn test_sw() {
        assert_eq!(singularize("sw", "vitabuni").unwrap(), "vitabu");
    }

    #[test]
    fn test_ta() {
        assert_eq!(singularize("ta", "pustakakal").unwrap(), "pustaka");
    }

    #[test]
    fn test_te() {
        assert_eq!(singularize("te", "pustakaalu").unwrap(), "pustakaa");
    }

    #[test]
    fn test_tg() {
        assert_eq!(singularize("tg", "ketabha").unwrap(), "ketab");
    }

    #[test]
    fn test_th() {
        assert_eq!(singularize("th", "user").unwrap(), "user");
    }

    #[test]
    fn test_ti() {
        assert_eq!(singularize("ti", "betoch").unwrap(), "bet");
    }

    #[test]
    fn test_tk() {
        assert_eq!(singularize("tk", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_tl() {
        assert_eq!(singularize("tl", "user").unwrap(), "user");
    }

    #[test]
    fn test_tn() {
        assert_eq!(singularize("tn", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_tr() {
        assert_eq!(singularize("tr", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_ts() {
        assert_eq!(singularize("ts", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_tt() {
        assert_eq!(singularize("tt", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_ug() {
        assert_eq!(singularize("ug", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_uk() {
        assert_eq!(singularize("uk", "produkty").unwrap(), "produkta");
    }

    #[test]
    fn test_ur() {
        assert_eq!(singularize("ur", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_uz() {
        assert_eq!(singularize("uz", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_ve() {
        assert_eq!(singularize("ve", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_vi() {
        assert_eq!(singularize("vi", "user").unwrap(), "user");
    }

    #[test]
    fn test_wo() {
        assert_eq!(singularize("wo", "user").unwrap(), "user");
    }

    #[test]
    fn test_xh() {
        assert_eq!(singularize("xh", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_yi() {
        assert_eq!(singularize("yi", "produktin").unwrap(), "produkt");
    }

    #[test]
    fn test_yo() {
        assert_eq!(singularize("yo", "user").unwrap(), "user");
    }

    #[test]
    fn test_zh() {
        assert_eq!(singularize("zh", "user").unwrap(), "user");
    }

    #[test]
    fn test_zu() {
        assert_eq!(singularize("zu", "izincwadini").unwrap(), "izincwadi");
    }

    #[test]
    fn test_unsupported_locale() {
        let err = singularize("xx", "users").unwrap_err();
        assert_eq!(err.to_string(), "unsupported locale: xx");
    }
}

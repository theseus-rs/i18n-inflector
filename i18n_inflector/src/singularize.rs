//! Singularization of nouns.

use alloc::borrow::Cow;
use alloc::format;

use phf::phf_map;

use crate::error::{Error, Result};
use crate::languages;

type SingularizeFn = for<'a> fn(&'a str) -> Cow<'a, str>;

static SINGULARIZE_MAP: phf::Map<&'static str, SingularizeFn> = phf_map! {
    "aa" => languages::aa::singularize,
    "ab" => languages::ab::singularize,
    "ae" => languages::ae::singularize,
    "af" => languages::af::singularize,
    "ak" => languages::ak::singularize,
    "am" => languages::am::singularize,
    "an" => languages::an::singularize,
    "ar" => languages::ar::singularize,
    "as" => languages::r#as::singularize,
    "av" => languages::av::singularize,
    "ay" => languages::ay::singularize,
    "az" => languages::az::singularize,
    "ba" => languages::ba::singularize,
    "be" => languages::be::singularize,
    "bg" => languages::bg::singularize,
    "bi" => languages::bi::singularize,
    "bm" => languages::bm::singularize,
    "bn" => languages::bn::singularize,
    "bo" => languages::bo::singularize,
    "br" => languages::br::singularize,
    "bs" => languages::bs::singularize,
    "ca" => languages::ca::singularize,
    "ce" => languages::ce::singularize,
    "ch" => languages::ch::singularize,
    "co" => languages::co::singularize,
    "cs" => languages::cs::singularize,
    "cu" => languages::cu::singularize,
    "cv" => languages::cv::singularize,
    "cy" => languages::cy::singularize,
    "da" => languages::da::singularize,
    "de" => languages::de::singularize,
    "dv" => languages::dv::singularize,
    "dz" => languages::dz::singularize,
    "ee" => languages::ee::singularize,
    "el" => languages::el::singularize,
    "en" => languages::en::singularize,
    "eo" => languages::eo::singularize,
    "es" => languages::es::singularize,
    "et" => languages::et::singularize,
    "eu" => languages::eu::singularize,
    "fa" => languages::fa::singularize,
    "ff" => languages::ff::singularize,
    "fi" => languages::fi::singularize,
    "fj" => languages::fj::singularize,
    "fo" => languages::fo::singularize,
    "fr" => languages::fr::singularize,
    "fy" => languages::fy::singularize,
    "ga" => languages::ga::singularize,
    "gd" => languages::gd::singularize,
    "gl" => languages::gl::singularize,
    "gn" => languages::gn::singularize,
    "gu" => languages::gu::singularize,
    "gv" => languages::gv::singularize,
    "ha" => languages::ha::singularize,
    "he" => languages::he::singularize,
    "hi" => languages::hi::singularize,
    "ho" => languages::ho::singularize,
    "hr" => languages::hr::singularize,
    "ht" => languages::ht::singularize,
    "hu" => languages::hu::singularize,
    "hy" => languages::hy::singularize,
    "ia" => languages::ia::singularize,
    "id" => languages::id::singularize,
    "ie" => languages::ie::singularize,
    "ig" => languages::ig::singularize,
    "ii" => languages::ii::singularize,
    "ik" => languages::ik::singularize,
    "is" => languages::is::singularize,
    "it" => languages::it::singularize,
    "iu" => languages::iu::singularize,
    "ja" => languages::ja::singularize,
    "jv" => languages::jv::singularize,
    "ka" => languages::ka::singularize,
    "kg" => languages::kg::singularize,
    "ki" => languages::ki::singularize,
    "kj" => languages::kj::singularize,
    "kk" => languages::kk::singularize,
    "km" => languages::km::singularize,
    "kn" => languages::kn::singularize,
    "ko" => languages::ko::singularize,
    "ku" => languages::ku::singularize,
    "kv" => languages::kv::singularize,
    "kw" => languages::kw::singularize,
    "ky" => languages::ky::singularize,
    "la" => languages::la::singularize,
    "lb" => languages::lb::singularize,
    "lg" => languages::lg::singularize,
    "li" => languages::li::singularize,
    "lo" => languages::lo::singularize,
    "lt" => languages::lt::singularize,
    "lu" => languages::lu::singularize,
    "lv" => languages::lv::singularize,
    "mg" => languages::mg::singularize,
    "mi" => languages::mi::singularize,
    "mk" => languages::mk::singularize,
    "ml" => languages::ml::singularize,
    "mn" => languages::mn::singularize,
    "mr" => languages::mr::singularize,
    "ms" => languages::ms::singularize,
    "mt" => languages::mt::singularize,
    "my" => languages::my::singularize,
    "nb" => languages::nb::singularize,
    "nd" => languages::nd::singularize,
    "ne" => languages::ne::singularize,
    "nl" => languages::nl::singularize,
    "nn" => languages::nn::singularize,
    "no" => languages::no::singularize,
    "nr" => languages::nr::singularize,
    "nv" => languages::nv::singularize,
    "ny" => languages::ny::singularize,
    "oc" => languages::oc::singularize,
    "oj" => languages::oj::singularize,
    "om" => languages::om::singularize,
    "or" => languages::or::singularize,
    "os" => languages::os::singularize,
    "pa" => languages::pa::singularize,
    "pi" => languages::pi::singularize,
    "pl" => languages::pl::singularize,
    "ps" => languages::ps::singularize,
    "pt" => languages::pt::singularize,
    "qu" => languages::qu::singularize,
    "rm" => languages::rm::singularize,
    "ro" => languages::ro::singularize,
    "ru" => languages::ru::singularize,
    "rw" => languages::rw::singularize,
    "sa" => languages::sa::singularize,
    "sc" => languages::sc::singularize,
    "sd" => languages::sd::singularize,
    "se" => languages::se::singularize,
    "sg" => languages::sg::singularize,
    "si" => languages::si::singularize,
    "sk" => languages::sk::singularize,
    "sl" => languages::sl::singularize,
    "sm" => languages::sm::singularize,
    "sn" => languages::sn::singularize,
    "so" => languages::so::singularize,
    "sq" => languages::sq::singularize,
    "sr" => languages::sr::singularize,
    "ss" => languages::ss::singularize,
    "st" => languages::st::singularize,
    "su" => languages::su::singularize,
    "sv" => languages::sv::singularize,
    "sw" => languages::sw::singularize,
    "ta" => languages::ta::singularize,
    "te" => languages::te::singularize,
    "tg" => languages::tg::singularize,
    "th" => languages::th::singularize,
    "ti" => languages::ti::singularize,
    "tk" => languages::tk::singularize,
    "tl" => languages::tl::singularize,
    "tn" => languages::tn::singularize,
    "tr" => languages::tr::singularize,
    "ts" => languages::ts::singularize,
    "tt" => languages::tt::singularize,
    "ug" => languages::ug::singularize,
    "uk" => languages::uk::singularize,
    "ur" => languages::ur::singularize,
    "uz" => languages::uz::singularize,
    "ve" => languages::ve::singularize,
    "vi" => languages::vi::singularize,
    "wa" => languages::wa::singularize,
    "wo" => languages::wo::singularize,
    "xh" => languages::xh::singularize,
    "yi" => languages::yi::singularize,
    "yo" => languages::yo::singularize,
    "zh" => languages::zh::singularize,
    "zu" => languages::zu::singularize,
};

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
pub fn singularize<'a>(locale: &str, name: &'a str) -> Result<Cow<'a, str>> {
    SINGULARIZE_MAP
        .get(locale)
        .map(|f| f(name))
        .ok_or_else(|| Error::new(format!("unsupported locale: {locale}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_aa() {
        assert_eq!(singularize("aa", "user").unwrap(), "user");
    }

    #[test]
    fn test_ab() {
        assert_eq!(singularize("ab", "user").unwrap(), "user");
    }

    #[test]
    fn test_ae() {
        assert_eq!(singularize("ae", "user").unwrap(), "user");
    }

    #[test]
    fn test_af() {
        assert_eq!(singularize("af", "honde").unwrap(), "hond");
    }

    #[test]
    fn test_ak() {
        assert_eq!(singularize("ak", "user").unwrap(), "user");
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
    fn test_av() {
        assert_eq!(singularize("av", "user").unwrap(), "user");
    }

    #[test]
    fn test_ay() {
        assert_eq!(singularize("ay", "utanaka").unwrap(), "uta");
    }

    #[test]
    fn test_az() {
        assert_eq!(singularize("az", "kullanicilar").unwrap(), "kullanici");
    }

    #[test]
    fn test_ba() {
        assert_eq!(singularize("ba", "kullanicilar").unwrap(), "kullanici");
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
    fn test_bi() {
        assert_eq!(singularize("bi", "user").unwrap(), "user");
    }

    #[test]
    fn test_bm() {
        assert_eq!(singularize("bm", "user").unwrap(), "user");
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
    fn test_ce() {
        assert_eq!(singularize("ce", "user").unwrap(), "user");
    }

    #[test]
    fn test_ch() {
        assert_eq!(singularize("ch", "user").unwrap(), "user");
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
    fn test_cu() {
        assert_eq!(singularize("cu", "user").unwrap(), "user");
    }

    #[test]
    fn test_cv() {
        assert_eq!(singularize("cv", "kullanicilar").unwrap(), "kullanici");
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
    fn test_dv() {
        assert_eq!(singularize("dv", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_dz() {
        assert_eq!(singularize("dz", "user").unwrap(), "user");
    }

    #[test]
    fn test_ee() {
        assert_eq!(singularize("ee", "user").unwrap(), "user");
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
    fn test_ff() {
        assert_eq!(singularize("ff", "user").unwrap(), "user");
    }

    #[test]
    fn test_fi() {
        assert_eq!(singularize("fi", "tuotteet").unwrap(), "tuottee");
    }

    #[test]
    fn test_fj() {
        assert_eq!(singularize("fj", "user").unwrap(), "user");
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
    fn test_gn() {
        assert_eq!(singularize("gn", "mitãkuéra").unwrap(), "mitã");
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
    fn test_ho() {
        assert_eq!(singularize("ho", "user").unwrap(), "user");
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
    fn test_ia() {
        assert_eq!(singularize("ia", "utilisateurs").unwrap(), "utilisateur");
    }

    #[test]
    fn test_id() {
        assert_eq!(singularize("id", "user").unwrap(), "user");
    }

    #[test]
    fn test_ie() {
        assert_eq!(singularize("ie", "utilisateurs").unwrap(), "utilisateur");
    }

    #[test]
    fn test_ig() {
        assert_eq!(singularize("ig", "user").unwrap(), "user");
    }

    #[test]
    fn test_ii() {
        assert_eq!(singularize("ii", "user").unwrap(), "user");
    }

    #[test]
    fn test_ik() {
        assert_eq!(singularize("ik", "user").unwrap(), "user");
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
    fn test_iu() {
        assert_eq!(singularize("iu", "user").unwrap(), "user");
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
    fn test_kg() {
        assert_eq!(singularize("kg", "vitabuni").unwrap(), "vitabu");
    }

    #[test]
    fn test_ki() {
        assert_eq!(singularize("ki", "vitabuni").unwrap(), "vitabu");
    }

    #[test]
    fn test_kj() {
        assert_eq!(singularize("kj", "vitabuni").unwrap(), "vitabu");
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
    fn test_kv() {
        assert_eq!(singularize("kv", "user").unwrap(), "user");
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
    fn test_lg() {
        assert_eq!(singularize("lg", "vitabuni").unwrap(), "vitabu");
    }

    #[test]
    fn test_li() {
        assert_eq!(singularize("li", "klanten").unwrap(), "klant");
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
    fn test_lu() {
        assert_eq!(singularize("lu", "vitabuni").unwrap(), "vitabu");
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
    fn test_nv() {
        assert_eq!(singularize("nv", "user").unwrap(), "user");
    }

    #[test]
    fn test_ny() {
        assert_eq!(singularize("ny", "vitabuni").unwrap(), "vitabu");
    }

    #[test]
    fn test_oc() {
        assert_eq!(singularize("oc", "utilisateurs").unwrap(), "utilisateur");
    }

    #[test]
    fn test_oj() {
        assert_eq!(singularize("oj", "user").unwrap(), "user");
    }

    #[test]
    fn test_om() {
        assert_eq!(singularize("om", "buugaagyo").unwrap(), "buugaag");
    }

    #[test]
    fn test_or() {
        assert_eq!(singularize("or", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_os() {
        assert_eq!(singularize("os", "ketabha").unwrap(), "ketab");
    }

    #[test]
    fn test_pa() {
        assert_eq!(singularize("pa", "upyogakartaon").unwrap(), "upyogakarta");
    }

    #[test]
    fn test_pi() {
        assert_eq!(singularize("pi", "user").unwrap(), "user");
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
    fn test_sa() {
        assert_eq!(singularize("sa", "user").unwrap(), "user");
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
    fn test_se() {
        assert_eq!(singularize("se", "user").unwrap(), "user");
    }

    #[test]
    fn test_sg() {
        assert_eq!(singularize("sg", "user").unwrap(), "user");
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
    fn test_wa() {
        assert_eq!(singularize("wa", "utilisateurs").unwrap(), "utilisateur");
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

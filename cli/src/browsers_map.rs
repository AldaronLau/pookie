use std::collections::HashMap;

use lazy_static::lazy_static;
use pookie::{self, enums::Cookie, Result};

// Incorrectly assumed unused
#[allow(dead_code)]
type BrowserFn = fn(Option<Vec<String>>) -> Result<Vec<Cookie>>;

lazy_static! {
    pub static ref BROWSERS_MAP: HashMap<String, BrowserFn> = {
        let mut map: HashMap<String, BrowserFn> = HashMap::default();

        map.insert("brave".into(), pookie::brave);

        #[cfg(target_os = "linux")]
        map.insert("cachy".into(), pookie::cachy);

        map.insert("chromium".into(), pookie::chromium);
        map.insert("chrome".into(), pookie::chrome);
        map.insert("edge".into(), pookie::edge);
        map.insert("firefox".into(), pookie::firefox);
        map.insert("zen".into(), pookie::zen);
        map.insert("librewolf".into(), pookie::librewolf);
        map.insert("opera".into(), pookie::opera);
        map.insert("opera gx".into(), pookie::opera_gx);

        #[cfg(target_os = "macos")]
        map.insert("safari".into(), pookie::safari);

        map.insert("vivaldi".into(), pookie::vivaldi);
        map.insert("arc".into(), pookie::arc);
        map
    };
}

lazy_static! {
    pub static ref BROWSERS_MAP_KEYS: Vec<&'static str> = {
        let keys = BROWSERS_MAP.keys();
        keys.map(|s| s.as_str()).collect()
    };
}

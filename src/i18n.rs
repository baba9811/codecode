use std::{collections::HashMap, sync::OnceLock};

pub const UI_LANGUAGES: &[&str] = &["en", "ko", "ja", "zh", "es"];

static EN: OnceLock<HashMap<String, String>> = OnceLock::new();
static KO: OnceLock<HashMap<String, String>> = OnceLock::new();
static JA: OnceLock<HashMap<String, String>> = OnceLock::new();
static ZH: OnceLock<HashMap<String, String>> = OnceLock::new();
static ES: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn normalize_ui_language(language: &str) -> String {
    let lower = language.trim().to_lowercase();
    let short = lower
        .split(['-', '_'])
        .next()
        .filter(|value| !value.is_empty())
        .unwrap_or("en");
    if UI_LANGUAGES.contains(&short) {
        short.to_string()
    } else {
        "en".to_string()
    }
}

pub fn ui_text(lang: &str, key: &str) -> &'static str {
    catalog(&normalize_ui_language(lang))
        .get(key)
        .or_else(|| catalog("en").get(key))
        .map(String::as_str)
        .unwrap_or("")
}

fn catalog(lang: &str) -> &'static HashMap<String, String> {
    match lang {
        "ko" => KO.get_or_init(|| load(include_str!("../assets/i18n/ko.json"))),
        "ja" => JA.get_or_init(|| load(include_str!("../assets/i18n/ja.json"))),
        "zh" => ZH.get_or_init(|| load(include_str!("../assets/i18n/zh.json"))),
        "es" => ES.get_or_init(|| load(include_str!("../assets/i18n/es.json"))),
        _ => EN.get_or_init(|| load(include_str!("../assets/i18n/en.json"))),
    }
}

fn load(text: &str) -> HashMap<String, String> {
    serde_json::from_str(text).expect("valid i18n catalog")
}

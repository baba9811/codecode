use super::*;

pub fn ext_for(language: &str) -> &'static str {
    match normalize_language(language).as_str() {
        "python" => "py",
        "ts" => "ts",
        "java" => "java",
        "rust" => "rs",
        _ => "py",
    }
}
pub fn problem_by_id<'a>(bank: &'a [Problem], problem_id: &str) -> Option<&'a Problem> {
    bank.iter().find(|problem| problem.id == problem_id)
}

pub fn normalize_language(language: &str) -> String {
    let language = language.trim().to_lowercase();
    if LANGUAGES.contains(&language.as_str()) {
        language
    } else {
        "python".to_string()
    }
}

pub fn parse_language_list(value: &str) -> Vec<String> {
    let mut languages = Vec::new();
    for language in value.split(',') {
        let language = language.trim().to_lowercase();
        if language == "all" {
            return Vec::new();
        }
        if LANGUAGES.contains(&language.as_str()) && !languages.contains(&language) {
            languages.push(language);
        }
    }
    languages
}

pub fn normalize_language_list(languages: &[String]) -> Vec<String> {
    parse_language_list(&languages.join(","))
}

pub fn parse_ui_language_list(value: &str) -> Vec<String> {
    let mut languages = Vec::new();
    for language in value.split(',') {
        let lower = language.trim().to_lowercase();
        if lower == "all" {
            return Vec::new();
        }
        let language = lower
            .split(['-', '_'])
            .next()
            .filter(|value| UI_LANGUAGES.contains(value))
            .unwrap_or("");
        if language == "all" {
            return Vec::new();
        }
        if !language.is_empty() && !languages.iter().any(|value| value == language) {
            languages.push(language.to_string());
        }
    }
    languages
}

pub fn normalize_ui_language_list(languages: &[String]) -> Vec<String> {
    parse_ui_language_list(&languages.join(","))
}

pub fn normalize_next_source(source: &str) -> String {
    if source == "ai" {
        "ai".to_string()
    } else {
        "bank".to_string()
    }
}

pub fn normalize_ai_provider(provider: &str) -> String {
    if provider == "claude" {
        "claude".to_string()
    } else {
        "codex".to_string()
    }
}

pub fn normalize_ai_effort(provider: &str, effort: &str) -> String {
    let effort = effort.trim().to_lowercase();
    let provider = normalize_ai_provider(provider);
    let allowed = if provider == "claude" {
        CLAUDE_AI_EFFORTS
    } else {
        CODEX_AI_EFFORTS
    };
    if allowed.contains(&effort.as_str()) {
        effort
    } else if provider == "codex" && effort == "max" {
        "xhigh".to_string()
    } else {
        "auto".to_string()
    }
}

pub fn template_for(language: &str) -> String {
    match normalize_language(language).as_str() {
        "python" => "# Read from stdin and print to stdout.\nimport sys\n\n\n".to_string(),
        "ts" => "const fs = require('fs');\nconst input = fs.readFileSync(0, 'utf8');\n\n".to_string(),
        "java" => "import java.io.*;\n\nclass Solution {\n    public static void main(String[] args) throws Exception {\n    }\n}\n".to_string(),
        "rust" => "fn main() {\n}\n".to_string(),
        _ => String::new(),
    }
}

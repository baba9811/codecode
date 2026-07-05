use crate::core::{AppState, DIFFICULTIES, LANGUAGES, THEMES, UI_LANGUAGES};

pub(super) struct SettingsChange {
    pub reload_editor: bool,
}

pub(super) fn row_count() -> usize {
    4 + LANGUAGES.len() + UI_LANGUAGES.len()
}

pub(super) fn render(state: &AppState, cursor: Option<usize>) -> String {
    let settings = &state.settings;
    let ui_language = settings.ui_language.as_str();
    let topics = list_or_none(&settings.topics, ui_language);
    let avoid = list_or_none(&settings.avoid_topics, ui_language);
    let generate_languages = list_or_all(&settings.generate_languages, ui_language);
    let generate_ui_languages = list_or_all(&settings.generate_ui_languages, ui_language);
    let mut lines = vec![
        label(ui_language, "title").to_string(),
        String::new(),
        label(ui_language, "instructions").to_string(),
        String::new(),
        row(
            cursor,
            0,
            &format!(
                "{}: {}",
                label(ui_language, "code_language"),
                settings.language
            ),
        ),
        row(
            cursor,
            1,
            &format!(
                "{}: {}",
                label(ui_language, "ui_language"),
                settings.ui_language
            ),
        ),
        row(
            cursor,
            2,
            &format!("{}: {}", label(ui_language, "theme"), settings.theme),
        ),
        row(
            cursor,
            3,
            &format!(
                "{}: {}",
                label(ui_language, "difficulty"),
                settings.difficulty
            ),
        ),
        String::new(),
        format!("{}: {topics}", label(ui_language, "preferred_topics")),
        format!("{}: {avoid}", label(ui_language, "avoid_topics")),
        format!(
            "{}: {generate_languages}",
            label(ui_language, "generated_answer_languages")
        ),
        format!(
            "{}: {generate_ui_languages}",
            label(ui_language, "generated_ui_languages")
        ),
        format!("AI provider: {}", settings.ai_provider),
        format!(
            "AI model: {}",
            if settings.ai_model == "auto" {
                label(ui_language, "provider_default")
            } else {
                settings.ai_model.as_str()
            }
        ),
        String::new(),
        label(ui_language, "answer_toggles").to_string(),
    ];
    for (index, language) in LANGUAGES.iter().enumerate() {
        let row_index = 4 + index;
        let checked = generate_language_enabled(state, language);
        lines.push(row(
            cursor,
            row_index,
            &format!("{} {language}", checkbox(checked)),
        ));
    }
    lines.push(String::new());
    lines.push(label(ui_language, "ui_toggles").to_string());
    for (index, language) in UI_LANGUAGES.iter().enumerate() {
        let row_index = 4 + LANGUAGES.len() + index;
        let checked = generate_ui_language_enabled(state, language);
        lines.push(row(
            cursor,
            row_index,
            &format!("{} {language}", checkbox(checked)),
        ));
    }
    lines.extend([
        String::new(),
        label(ui_language, "commands").to_string(),
        "/profile".to_string(),
        "/difficulty auto|easy|medium|hard".to_string(),
        "/topics arrays, strings".to_string(),
        "/avoid dp, graph".to_string(),
        "/generate-languages all|python, rust".to_string(),
        "/generate-ui all|en, ko".to_string(),
        "/provider codex|claude".to_string(),
        "/model auto".to_string(),
    ]);
    lines.join("\n")
}

fn label<'a>(ui_language: &str, key: &'a str) -> &'a str {
    if ui_language == "ko" {
        match key {
            "title" => "사용자 프로필",
            "instructions" => "위/아래로 이동하고 Space 또는 Enter로 변경/토글",
            "code_language" => "코드 언어",
            "ui_language" => "UI 언어",
            "theme" => "테마",
            "difficulty" => "난이도",
            "preferred_topics" => "선호 주제",
            "avoid_topics" => "피할 주제",
            "generated_answer_languages" => "생성 정답 언어",
            "generated_ui_languages" => "생성 문제 언어",
            "provider_default" => "auto (provider 기본값)",
            "answer_toggles" => "생성 정답 언어 토글",
            "ui_toggles" => "생성 문제 언어 토글",
            "commands" => "명령",
            "none" => "(없음)",
            "all" => "전체",
            _ => key,
        }
    } else {
        match key {
            "title" => "User profile",
            "instructions" => "Use up/down to move. Press Space or Enter to cycle/toggle.",
            "code_language" => "Code language",
            "ui_language" => "UI language",
            "theme" => "Theme",
            "difficulty" => "Difficulty",
            "preferred_topics" => "Preferred topics",
            "avoid_topics" => "Avoid topics",
            "generated_answer_languages" => "Generated answer languages",
            "generated_ui_languages" => "Generated UI languages",
            "provider_default" => "auto (provider default)",
            "answer_toggles" => "Generated answer language toggles",
            "ui_toggles" => "Generated problem text language toggles",
            "commands" => "Commands",
            "none" => "(none)",
            "all" => "all",
            _ => key,
        }
    }
}

pub(super) fn apply_selected(state: &mut AppState, selected: usize) -> SettingsChange {
    let mut reload_editor = false;
    match selected {
        0 => {
            let current = LANGUAGES
                .iter()
                .position(|language| language == &state.settings.language)
                .unwrap_or(0);
            state.settings.language = LANGUAGES[(current + 1) % LANGUAGES.len()].to_string();
            reload_editor = true;
        }
        1 => {
            let current = UI_LANGUAGES
                .iter()
                .position(|language| language == &state.settings.ui_language)
                .unwrap_or(0);
            state.settings.ui_language =
                UI_LANGUAGES[(current + 1) % UI_LANGUAGES.len()].to_string();
        }
        2 => {
            let current = THEMES
                .iter()
                .position(|theme| theme == &state.settings.theme)
                .unwrap_or(0);
            state.settings.theme = THEMES[(current + 1) % THEMES.len()].to_string();
        }
        3 => {
            let current = DIFFICULTIES
                .iter()
                .position(|difficulty| difficulty == &state.settings.difficulty)
                .unwrap_or(0);
            let difficulty = DIFFICULTIES[(current + 1) % DIFFICULTIES.len()].to_string();
            state.settings.difficulty = difficulty.clone();
            if difficulty != "auto" {
                state.suggested_next_difficulty = difficulty;
            }
        }
        row if row < 4 + LANGUAGES.len() => {
            toggle_generate_language(state, LANGUAGES[row - 4]);
        }
        row if row < row_count() => {
            toggle_generate_ui_language(state, UI_LANGUAGES[row - 4 - LANGUAGES.len()]);
        }
        _ => {}
    }
    SettingsChange { reload_editor }
}

fn row(cursor: Option<usize>, index: usize, text: &str) -> String {
    let marker = if cursor == Some(index) { ">" } else { " " };
    format!("{marker} {text}")
}

fn generate_language_enabled(state: &AppState, language: &str) -> bool {
    state.settings.generate_languages.is_empty()
        || state
            .settings
            .generate_languages
            .iter()
            .any(|value| value == language)
}

fn generate_ui_language_enabled(state: &AppState, language: &str) -> bool {
    state.settings.generate_ui_languages.is_empty()
        || state
            .settings
            .generate_ui_languages
            .iter()
            .any(|value| value == language)
}

fn toggle_generate_language(state: &mut AppState, language: &str) {
    if state.settings.generate_languages.is_empty() {
        state.settings.generate_languages = LANGUAGES
            .iter()
            .filter(|value| **value != language)
            .map(|value| (*value).to_string())
            .collect();
        return;
    }
    if generate_language_enabled(state, language) {
        if state.settings.generate_languages.len() > 1 {
            state
                .settings
                .generate_languages
                .retain(|value| value != language);
        }
    } else {
        state.settings.generate_languages.push(language.to_string());
        state.settings.generate_languages = LANGUAGES
            .iter()
            .filter(|value| {
                state
                    .settings
                    .generate_languages
                    .iter()
                    .any(|selected| selected == *value)
            })
            .map(|value| (*value).to_string())
            .collect();
        if state.settings.generate_languages.len() == LANGUAGES.len() {
            state.settings.generate_languages.clear();
        }
    }
}

fn toggle_generate_ui_language(state: &mut AppState, language: &str) {
    if state.settings.generate_ui_languages.is_empty() {
        state.settings.generate_ui_languages = UI_LANGUAGES
            .iter()
            .filter(|value| **value != language)
            .map(|value| (*value).to_string())
            .collect();
        return;
    }
    if generate_ui_language_enabled(state, language) {
        if state.settings.generate_ui_languages.len() > 1 {
            state
                .settings
                .generate_ui_languages
                .retain(|value| value != language);
        }
    } else {
        state
            .settings
            .generate_ui_languages
            .push(language.to_string());
        state.settings.generate_ui_languages = UI_LANGUAGES
            .iter()
            .filter(|value| {
                state
                    .settings
                    .generate_ui_languages
                    .iter()
                    .any(|selected| selected == *value)
            })
            .map(|value| (*value).to_string())
            .collect();
        if state.settings.generate_ui_languages.len() == UI_LANGUAGES.len() {
            state.settings.generate_ui_languages.clear();
        }
    }
}

fn list_or_none(values: &[String], ui_language: &str) -> String {
    if values.is_empty() {
        label(ui_language, "none").to_string()
    } else {
        values.join(", ")
    }
}

fn list_or_all(values: &[String], ui_language: &str) -> String {
    if values.is_empty() {
        label(ui_language, "all").to_string()
    } else {
        values.join(", ")
    }
}

fn checkbox(checked: bool) -> &'static str {
    if checked { "[x]" } else { "[ ]" }
}

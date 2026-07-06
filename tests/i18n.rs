use practicode::core::{LANGUAGES, syntax_lessons_for};
use practicode::i18n::{UI_LANGUAGES, normalize_ui_language, ui_text};
use serde_json::Value;
use std::{fs, path::Path};

fn lesson_asset_dir(language: &str) -> &str {
    match language {
        "ts" => "typescript",
        _ => language,
    }
}

fn assert_no_english_scaffolding_terms(ui_language: &str, lesson_id: &str, text: &str) {
    if ui_language == "en" {
        return;
    }
    let text = text.to_ascii_lowercase();
    for term in ["worked example", "starter"] {
        assert!(
            !text.contains(term),
            "{ui_language}:{lesson_id} contains untranslated scaffolding term: {term}"
        );
    }
}

#[test]
fn ui_catalogs_load_and_fallback_to_english() {
    for lang in UI_LANGUAGES {
        assert!(!ui_text(lang, "cmd_run").is_empty(), "{lang}");
        assert!(!ui_text(lang, "cmd_home").is_empty(), "{lang}");
        assert!(!ui_text(lang, "cmd_doctor").is_empty(), "{lang}");
        assert!(!ui_text(lang, "home_learn_choice").is_empty(), "{lang}");
        assert!(!ui_text(lang, "home_practice_choice").is_empty(), "{lang}");
        assert!(!ui_text(lang, "update_available").is_empty(), "{lang}");
    }
    assert_eq!(normalize_ui_language("zh-CN"), "zh");
    assert_eq!(ui_text("xx", "cmd_run"), "Judge the current submission");
}

#[test]
fn ui_catalogs_do_not_store_syntax_curriculum_copy() {
    for ui_language in UI_LANGUAGES {
        for language in LANGUAGES {
            for lesson in syntax_lessons_for(language) {
                let id = lesson.id.replace('-', "_");
                assert!(
                    ui_text(ui_language, &format!("syntax_{id}_title")).is_empty(),
                    "{ui_language}:{id}:title"
                );
                assert!(
                    ui_text(ui_language, &format!("syntax_{id}_body")).is_empty(),
                    "{ui_language}:{id}:body"
                );
            }
        }
    }
}

#[test]
fn lesson_catalogs_have_complete_study_copy_for_every_language() {
    for &ui_language in UI_LANGUAGES {
        let legacy_path = format!("assets/lessons/{ui_language}.json");
        assert!(
            !Path::new(&legacy_path).exists(),
            "legacy lesson catalog should be removed: {legacy_path}"
        );
    }

    for &ui_language in UI_LANGUAGES {
        for &language in LANGUAGES {
            let path = format!(
                "assets/lessons/{}/{ui_language}.json",
                lesson_asset_dir(language)
            );
            let catalog: Value =
                serde_json::from_str(&fs::read_to_string(&path).unwrap()).expect(&path);
            assert_eq!(
                catalog
                    .get("schema_version")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
                1,
                "{path}: schema_version"
            );
            assert_eq!(
                catalog
                    .get("programming_language")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                language,
                "{path}: programming_language"
            );
            assert_eq!(
                catalog
                    .get("ui_language")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                ui_language,
                "{path}: ui_language"
            );
            let lessons = catalog
                .get("lessons")
                .and_then(Value::as_object)
                .unwrap_or_else(|| panic!("{path}: missing lessons object"));
            for lesson in syntax_lessons_for(language) {
                let copy = lessons.get(lesson.id).unwrap_or_else(|| {
                    panic!("{ui_language}: missing lesson copy for {}", lesson.id)
                });
                for field in ["title", "concept", "worked_example", "exercise_prompt"] {
                    let text = copy
                        .get(field)
                        .and_then(Value::as_str)
                        .unwrap_or_else(|| panic!("{ui_language}:{} missing {field}", lesson.id));
                    assert!(
                        text.chars().count() >= 45 || field == "title",
                        "{ui_language}:{} {field} too short: {text}",
                        lesson.id
                    );
                    assert_no_english_scaffolding_terms(ui_language, lesson.id, text);
                }
                for field in ["common_mistakes", "self_check"] {
                    let items = copy
                        .get(field)
                        .and_then(Value::as_array)
                        .unwrap_or_else(|| panic!("{ui_language}:{} missing {field}", lesson.id));
                    assert!(
                        items.len() >= 2,
                        "{ui_language}:{} {field} needs at least 2 items",
                        lesson.id
                    );
                    for item in items {
                        let text = item
                            .as_str()
                            .unwrap_or_else(|| panic!("{ui_language}:{} bad {field}", lesson.id));
                        assert!(
                            text.chars().count() >= 12,
                            "{ui_language}:{} {field} item too short: {text}",
                            lesson.id
                        );
                        assert_no_english_scaffolding_terms(ui_language, lesson.id, text);
                    }
                }
            }
        }
    }
}

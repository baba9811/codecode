use practicode::core::{LANGUAGES, syntax_lessons_for};
use practicode::i18n::{UI_LANGUAGES, normalize_ui_language, ui_text};
use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};

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

fn assert_not_generic_lesson_copy(ui_language: &str, lesson_id: &str, text: &str) {
    for phrase in [
        "is easiest to learn by tracing the value flow",
        "with the smallest code shape",
        "Memorizing the example instead of explaining the value flow",
        "What value exists immediately before",
        "문법 이름을 외우기보다 예제의 값 흐름",
        "가장 작은 관찰 가능한 코드",
        "用語を暗記するよりも",
        "最小限のコードで示しています",
        "不要只记语法名称",
        "最小但可观察的代码",
        "se aprende mejor siguiendo el recorrido del valor",
        "con el código mínimo que produce un resultado observable",
    ] {
        assert!(
            !text.contains(phrase),
            "{ui_language}:{lesson_id} contains generic lesson-copy phrase: {phrase}"
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
            let catalog_object = catalog
                .as_object()
                .unwrap_or_else(|| panic!("{path}: catalog should be an object"));
            for key in catalog_object.keys() {
                assert!(
                    [
                        "schema_version",
                        "programming_language",
                        "ui_language",
                        "lessons"
                    ]
                    .contains(&key.as_str()),
                    "{path}: unexpected top-level key {key}"
                );
            }
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
            assert_eq!(
                lessons.len(),
                syntax_lessons_for(language).len(),
                "{path}: unexpected lesson count"
            );
            for lesson in syntax_lessons_for(language) {
                let copy = lessons.get(lesson.id).unwrap_or_else(|| {
                    panic!("{ui_language}: missing lesson copy for {}", lesson.id)
                });
                let copy_object = copy.as_object().unwrap_or_else(|| {
                    panic!("{ui_language}:{} copy should be an object", lesson.id)
                });
                for key in copy_object.keys() {
                    assert!(
                        [
                            "title",
                            "concept",
                            "worked_example",
                            "common_mistakes",
                            "self_check",
                            "exercise_prompt",
                        ]
                        .contains(&key.as_str()),
                        "{ui_language}:{} unexpected lesson-copy key {key}",
                        lesson.id
                    );
                }
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
                    assert_not_generic_lesson_copy(ui_language, lesson.id, text);
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
                        assert_not_generic_lesson_copy(ui_language, lesson.id, text);
                    }
                }
            }

            let mut repeated = HashMap::<String, usize>::new();
            for copy in lessons.values() {
                let copy = copy.as_object().expect("lesson copy object");
                for value in copy.values() {
                    match value {
                        Value::String(text) => {
                            let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
                            if text.chars().count() >= 45 {
                                *repeated.entry(text).or_default() += 1;
                            }
                        }
                        Value::Array(items) => {
                            for item in items {
                                let text = item
                                    .as_str()
                                    .expect("lesson list item string")
                                    .split_whitespace()
                                    .collect::<Vec<_>>()
                                    .join(" ");
                                if text.chars().count() >= 45 {
                                    *repeated.entry(text).or_default() += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            for (text, count) in repeated {
                assert!(
                    count <= 3,
                    "{path}: repeated lesson copy {count} times: {text}"
                );
            }
        }
    }
}

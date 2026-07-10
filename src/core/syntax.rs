use super::*;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct SyntaxLessonCopy {
    title: String,
    concept: String,
    worked_example: String,
    common_mistakes: Vec<String>,
    self_check: Vec<String>,
    exercise_prompt: String,
}

#[derive(Debug, Deserialize)]
struct SyntaxLessonCatalog {
    schema_version: u8,
    #[serde(rename = "programming_language")]
    _programming_language: String,
    #[serde(rename = "ui_language")]
    _ui_language: String,
    lessons: HashMap<String, SyntaxLessonCopy>,
}

type SyntaxLessonCopyMap = HashMap<String, SyntaxLessonCopy>;

static PY_EN_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static PY_KO_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static PY_JA_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static PY_ZH_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static PY_ES_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static TS_EN_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static TS_KO_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static TS_JA_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static TS_ZH_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static TS_ES_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static JAVA_EN_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static JAVA_KO_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static JAVA_JA_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static JAVA_ZH_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static JAVA_ES_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static RUST_EN_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static RUST_KO_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static RUST_JA_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static RUST_ZH_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();
static RUST_ES_LESSONS: OnceLock<SyntaxLessonCopyMap> = OnceLock::new();

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SyntaxTrack {
    Core,
    Lab,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SyntaxKind {
    Lesson,
    Checkpoint,
    Capstone,
}

#[derive(Clone, Copy, Debug)]
pub struct SyntaxCase {
    pub input: &'static str,
    pub output: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct SyntaxExercise {
    pub prompt: &'static str,
    pub starter: &'static str,
    pub cases: &'static [SyntaxCase],
}

#[derive(Clone, Copy, Debug)]
pub struct SyntaxLesson {
    pub id: &'static str,
    pub aliases: &'static [&'static str],
    pub language: &'static str,
    pub track: SyntaxTrack,
    pub kind: SyntaxKind,
    pub level: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub example: &'static str,
    pub exercise: SyntaxExercise,
    pub refs: &'static [&'static str],
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SyntaxCourseAsset {
    schema_version: u8,
    runtime: String,
    lessons: Vec<SyntaxLessonAsset>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SyntaxLessonAsset {
    id: String,
    aliases: Vec<String>,
    track: SyntaxTrack,
    kind: SyntaxKind,
    level: String,
    title: String,
    body: String,
    example: String,
    starter: String,
    cases: Vec<SyntaxCaseAsset>,
    refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SyntaxCaseAsset {
    input: String,
    output: String,
}

static PYTHON_COURSE: OnceLock<Vec<SyntaxLesson>> = OnceLock::new();
static TS_COURSE: OnceLock<Vec<SyntaxLesson>> = OnceLock::new();
static JAVA_COURSE: OnceLock<Vec<SyntaxLesson>> = OnceLock::new();
static RUST_COURSE: OnceLock<Vec<SyntaxLesson>> = OnceLock::new();

const SYNTAX_EXERCISE_PROMPT: &str = "Before you run, predict the output. Then run the starter and edit it until the expected output matches.";

fn load_course(path: &str, text: &str, runtime: &'static str) -> Vec<SyntaxLesson> {
    let catalog: SyntaxCourseAsset = serde_json::from_str(text)
        .unwrap_or_else(|error| panic!("invalid embedded syntax course {path}: {error}"));
    assert_eq!(catalog.schema_version, 1, "unsupported schema in {path}");
    assert_eq!(catalog.runtime, runtime, "unexpected runtime in {path}");
    catalog
        .lessons
        .into_iter()
        .map(|lesson| lesson.into_lesson(runtime))
        .collect()
}

impl SyntaxLessonAsset {
    fn into_lesson(self, language: &'static str) -> SyntaxLesson {
        SyntaxLesson {
            id: leak_string(self.id),
            aliases: leak_vec(self.aliases.into_iter().map(leak_string).collect()),
            language,
            track: self.track,
            kind: self.kind,
            level: leak_string(self.level),
            title: leak_string(self.title),
            body: leak_string(self.body),
            example: leak_string(self.example),
            exercise: SyntaxExercise {
                prompt: SYNTAX_EXERCISE_PROMPT,
                starter: leak_string(self.starter),
                cases: leak_vec(
                    self.cases
                        .into_iter()
                        .map(|case| SyntaxCase {
                            input: leak_string(case.input),
                            output: leak_string(case.output),
                        })
                        .collect(),
                ),
            },
            refs: leak_vec(self.refs.into_iter().map(leak_string).collect()),
        }
    }
}

fn leak_string(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn leak_vec<T>(values: Vec<T>) -> &'static [T] {
    Box::leak(values.into_boxed_slice())
}

pub fn syntax_lessons_for(language: &str) -> Vec<&'static SyntaxLesson> {
    let lessons = match normalize_language(language).as_str() {
        "ts" => TS_COURSE.get_or_init(|| {
            load_course(
                "assets/lessons/typescript/course.json",
                include_str!("../../assets/lessons/typescript/course.json"),
                "ts",
            )
        }),
        "java" => JAVA_COURSE.get_or_init(|| {
            load_course(
                "assets/lessons/java/course.json",
                include_str!("../../assets/lessons/java/course.json"),
                "java",
            )
        }),
        "rust" => RUST_COURSE.get_or_init(|| {
            load_course(
                "assets/lessons/rust/course.json",
                include_str!("../../assets/lessons/rust/course.json"),
                "rust",
            )
        }),
        _ => PYTHON_COURSE.get_or_init(|| {
            load_course(
                "assets/lessons/python/course.json",
                include_str!("../../assets/lessons/python/course.json"),
                "python",
            )
        }),
    };
    lessons.iter().collect()
}

pub fn current_syntax_lesson(state: &AppState, language: &str) -> &'static SyntaxLesson {
    let language = normalize_language(language);
    let lessons = syntax_lessons_for(&language);
    if let Some(id) = state.current_syntax_lesson.get(&language)
        && let Some(lesson) = lessons.iter().find(|lesson| lesson.id == id)
    {
        return lesson;
    }
    lessons
        .iter()
        .find(|lesson| !syntax_lesson_completed(state, &language, lesson.id))
        .copied()
        .unwrap_or(lessons[0])
}

pub fn syntax_progress_count(state: &AppState, language: &str) -> (usize, usize) {
    let language = normalize_language(language);
    (
        state
            .syntax_progress
            .get(&language)
            .map_or(0, |ids| ids.len()),
        syntax_lessons_for(&language).len(),
    )
}

pub fn syntax_lesson_completed(state: &AppState, language: &str, lesson_id: &str) -> bool {
    let language = normalize_language(language);
    state
        .syntax_progress
        .get(&language)
        .is_some_and(|ids| ids.iter().any(|id| id == lesson_id))
}

pub fn record_syntax_pass(state: &mut AppState, language: &str, lesson_id: &str) {
    let language = normalize_language(language);
    if !syntax_lessons_for(&language)
        .iter()
        .any(|lesson| lesson.id == lesson_id)
    {
        return;
    }
    let mut ids = state.syntax_progress.remove(&language).unwrap_or_default();
    if !ids.iter().any(|id| id == lesson_id) {
        ids.push(lesson_id.to_string());
    }
    state
        .syntax_progress
        .insert(language.clone(), normalize_syntax_ids_for(&language, &ids));
}

pub fn set_current_syntax_lesson(state: &mut AppState, language: &str, lesson_id: &str) {
    let language = normalize_language(language);
    if syntax_lessons_for(&language)
        .iter()
        .any(|lesson| lesson.id == lesson_id)
    {
        state
            .current_syntax_lesson
            .insert(language, lesson_id.to_string());
    }
}

pub fn next_syntax_lesson(state: &mut AppState, language: &str, direction: isize) {
    let language = normalize_language(language);
    let lessons = syntax_lessons_for(&language);
    let current = current_syntax_lesson(state, &language).id;
    let index = lessons
        .iter()
        .position(|lesson| lesson.id == current)
        .unwrap_or(0);
    let next = (index as isize + direction).clamp(0, lessons.len() as isize - 1) as usize;
    state
        .current_syntax_lesson
        .insert(language, lessons[next].id.to_string());
}

pub fn normalize_syntax_progress(
    progress: &HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut normalized = HashMap::new();
    for language in LANGUAGES {
        if let Some(ids) = progress.get(*language) {
            let ids = normalize_syntax_ids_for(language, ids);
            if !ids.is_empty() {
                normalized.insert((*language).to_string(), ids);
            }
        }
    }
    normalized
}

pub fn normalize_current_syntax_lessons(
    current: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut normalized = HashMap::new();
    for language in LANGUAGES {
        if let Some(id) = current.get(*language)
            && syntax_lessons_for(language)
                .iter()
                .any(|lesson| lesson.id == id)
        {
            normalized.insert((*language).to_string(), id.clone());
        }
    }
    normalized
}

pub fn ensure_syntax_submission(root: &Path, lesson: &SyntaxLesson) -> Result<PathBuf> {
    let path = root
        .join("submissions")
        .join(".syntax")
        .join(lesson.language)
        .join(lesson.id)
        .join(format!("exercise.{}", ext_for(lesson.language)));
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, lesson.exercise.starter)?;
    }
    Ok(path)
}

pub fn syntax_cases(lesson: &SyntaxLesson) -> Vec<IoCase> {
    lesson
        .exercise
        .cases
        .iter()
        .map(|case| IoCase {
            input: case.input.to_string(),
            output: case.output.to_string(),
        })
        .collect()
}

pub fn render_syntax_lesson(lesson: &SyntaxLesson, state: &AppState) -> String {
    let ui_language = &state.settings.ui_language;
    let (done, total) = syntax_progress_count(state, lesson.language);
    let completed = if syntax_lesson_completed(state, lesson.language, lesson.id) {
        ui_text(ui_language, "syntax_complete")
    } else {
        ui_text(ui_language, "syntax_open")
    };
    let refs = lesson.refs.join("\n");
    let concept = localized_syntax_body(lesson, ui_language);
    let worked_example = localized_syntax_worked_example(lesson, ui_language);
    let common_mistakes =
        localized_syntax_list_section(lesson, ui_language, "syntax_common_mistakes", |copy| {
            &copy.common_mistakes
        });
    let self_check =
        localized_syntax_list_section(lesson, ui_language, "syntax_self_check", |copy| {
            &copy.self_check
        });
    let extra_sections = [common_mistakes, self_check]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join("\n\n");
    let extra_sections = if extra_sections.is_empty() {
        String::new()
    } else {
        format!("\n\n{extra_sections}")
    };
    let exercise_goal = syntax_exercise_goal(lesson, ui_language);
    format!(
        "# {}: {}\n\n{}: {}\n{}: {}\n{}: {done}/{total} ({completed})\n\n## {}\n\n{}\n\n## {}\n\n{}\n\n## {}\n\n{}\n{}\n\n## {}\n\n{}",
        ui_text(ui_language, "syntax"),
        localized_syntax_title(lesson, ui_language),
        ui_text(ui_language, "syntax_language"),
        syntax_language_name(lesson.language),
        ui_text(ui_language, "syntax_level"),
        localized_syntax_level(lesson.level, ui_language),
        ui_text(ui_language, "syntax_progress"),
        ui_text(ui_language, "syntax_concept"),
        concept,
        ui_text(ui_language, "syntax_worked_example"),
        worked_example,
        ui_text(ui_language, "syntax_exercise"),
        exercise_goal,
        extra_sections,
        ui_text(ui_language, "syntax_references"),
        refs
    )
}

fn syntax_exercise_goal(lesson: &SyntaxLesson, ui_language: &str) -> String {
    let prompt = localized_syntax_exercise_prompt(lesson, ui_language);
    let Some(case) = lesson.exercise.cases.first() else {
        return prompt;
    };
    format!(
        "{}\n\n{}\n\n{}\n\n{}\n\n{}",
        prompt,
        ui_text(ui_language, "input"),
        fenced_text(case.input),
        ui_text(ui_language, "output"),
        fenced_text(case.output)
    )
}

pub fn syntax_lesson_study_context(lesson: &SyntaxLesson, ui_language: &str) -> String {
    let common_mistakes =
        localized_syntax_list_section(lesson, ui_language, "syntax_common_mistakes", |copy| {
            &copy.common_mistakes
        });
    let self_check =
        localized_syntax_list_section(lesson, ui_language, "syntax_self_check", |copy| {
            &copy.self_check
        });
    [
        format!(
            "Lesson: {} ({})",
            localized_syntax_title(lesson, ui_language),
            lesson.id
        ),
        format!("Concept:\n{}", localized_syntax_body(lesson, ui_language)),
        format!(
            "Worked example:\n{}",
            localized_syntax_worked_example(lesson, ui_language)
        ),
        common_mistakes.unwrap_or_default(),
        self_check.unwrap_or_default(),
        format!(
            "Exercise prompt:\n{}",
            localized_syntax_exercise_prompt(lesson, ui_language)
        ),
        format!("References:\n{}", lesson.refs.join("\n")),
    ]
    .into_iter()
    .filter(|section| !section.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n\n")
}

pub fn syntax_language_name(language: &str) -> &'static str {
    match normalize_language(language).as_str() {
        "ts" => "TypeScript",
        "java" => "Java",
        "rust" => "Rust",
        _ => "Python",
    }
}

fn localized_syntax_level(level: &'static str, ui_language: &str) -> &'static str {
    match level {
        "basic" => ui_text(ui_language, "syntax_basic"),
        "intermediate" => ui_text(ui_language, "syntax_intermediate"),
        "advanced" => ui_text(ui_language, "syntax_advanced"),
        _ => level,
    }
}

fn localized_syntax_exercise_prompt(lesson: &SyntaxLesson, ui_language: &str) -> String {
    required_lesson_copy_for(lesson, ui_language)
        .exercise_prompt
        .clone()
}

fn localized_syntax_title(lesson: &SyntaxLesson, ui_language: &str) -> String {
    required_lesson_copy_for(lesson, ui_language).title.clone()
}

fn localized_syntax_body(lesson: &SyntaxLesson, ui_language: &str) -> String {
    required_lesson_copy_for(lesson, ui_language)
        .concept
        .clone()
}

fn localized_syntax_worked_example(lesson: &SyntaxLesson, ui_language: &str) -> String {
    let mut text = String::new();
    text.push_str(&required_lesson_copy_for(lesson, ui_language).worked_example);
    text.push_str("\n\n");
    text.push_str(&format!("```{}\n{}\n```", lesson.language, lesson.example));
    text
}

fn localized_syntax_list_section(
    lesson: &SyntaxLesson,
    ui_language: &str,
    title_key: &str,
    items: fn(&SyntaxLessonCopy) -> &Vec<String>,
) -> Option<String> {
    let copy = required_lesson_copy_for(lesson, ui_language);
    let items = items(copy);
    if items.is_empty() {
        return None;
    }
    let body = items
        .iter()
        .map(|item| format!("- {item}"))
        .collect::<Vec<_>>()
        .join("\n");
    Some(format!("## {}\n\n{body}", ui_text(ui_language, title_key)))
}

fn required_lesson_copy_for(lesson: &SyntaxLesson, ui_language: &str) -> &'static SyntaxLessonCopy {
    let language = normalize_language(lesson.language);
    let ui_language = normalize_ui_language(ui_language);
    let catalog = match (language.as_str(), ui_language.as_str()) {
        ("python", "ko") => PY_KO_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/python/ko.json"))),
        ("python", "ja") => PY_JA_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/python/ja.json"))),
        ("python", "zh") => PY_ZH_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/python/zh.json"))),
        ("python", "es") => PY_ES_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/python/es.json"))),
        ("python", _) => PY_EN_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/python/en.json"))),
        ("ts", "ko") => TS_KO_LESSONS.get_or_init(|| {
            load_lesson_copy(include_str!("../../assets/lessons/typescript/ko.json"))
        }),
        ("ts", "ja") => TS_JA_LESSONS.get_or_init(|| {
            load_lesson_copy(include_str!("../../assets/lessons/typescript/ja.json"))
        }),
        ("ts", "zh") => TS_ZH_LESSONS.get_or_init(|| {
            load_lesson_copy(include_str!("../../assets/lessons/typescript/zh.json"))
        }),
        ("ts", "es") => TS_ES_LESSONS.get_or_init(|| {
            load_lesson_copy(include_str!("../../assets/lessons/typescript/es.json"))
        }),
        ("ts", _) => TS_EN_LESSONS.get_or_init(|| {
            load_lesson_copy(include_str!("../../assets/lessons/typescript/en.json"))
        }),
        ("java", "ko") => JAVA_KO_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/java/ko.json"))),
        ("java", "ja") => JAVA_JA_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/java/ja.json"))),
        ("java", "zh") => JAVA_ZH_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/java/zh.json"))),
        ("java", "es") => JAVA_ES_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/java/es.json"))),
        ("java", _) => JAVA_EN_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/java/en.json"))),
        ("rust", "ko") => RUST_KO_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/rust/ko.json"))),
        ("rust", "ja") => RUST_JA_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/rust/ja.json"))),
        ("rust", "zh") => RUST_ZH_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/rust/zh.json"))),
        ("rust", "es") => RUST_ES_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/rust/es.json"))),
        ("rust", _) => RUST_EN_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/rust/en.json"))),
        _ => PY_EN_LESSONS
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/python/en.json"))),
    };
    catalog.get(lesson.id).unwrap_or_else(|| {
        panic!(
            "missing lesson copy: {language}:{ui_language}:{}",
            lesson.id
        )
    })
}

fn load_lesson_copy(text: &str) -> SyntaxLessonCopyMap {
    let catalog: SyntaxLessonCatalog =
        serde_json::from_str(text).expect("valid syntax lesson copy");
    assert_eq!(
        catalog.schema_version, 1,
        "unsupported syntax lesson schema"
    );
    catalog.lessons
}

fn normalize_syntax_ids_for(language: &str, ids: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();
    for lesson in syntax_lessons_for(language) {
        if ids.iter().any(|id| id == lesson.id) && !normalized.iter().any(|id| id == lesson.id) {
            normalized.push(lesson.id.to_string());
        }
    }
    normalized
}

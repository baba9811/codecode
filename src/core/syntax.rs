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

static PYTHON_EN_COPY: OnceLock<HashMap<String, SyntaxLessonCopy>> = OnceLock::new();
static PYTHON_KO_COPY: OnceLock<HashMap<String, SyntaxLessonCopy>> = OnceLock::new();

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
    pub language: &'static str,
    pub level: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub example: &'static str,
    pub exercise: SyntaxExercise,
    pub refs: &'static [&'static str],
}

macro_rules! lesson {
    ($id:expr, $language:expr, $level:expr, $title:expr, $body:expr, $example:expr, $starter:expr, $cases:expr, $refs:expr) => {
        SyntaxLesson {
            id: $id,
            language: $language,
            level: $level,
            title: $title,
            body: $body,
            example: $example,
            exercise: SyntaxExercise {
                prompt: "Before you run, predict the output. Then run the starter and edit it until the expected output matches.",
                starter: $starter,
                cases: $cases,
            },
            refs: $refs,
        }
    };
}

const PY_REFS: &[&str] = &["https://docs.python.org/3/tutorial/index.html"];
const TS_REFS: &[&str] = &[
    "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide",
    "https://www.typescriptlang.org/docs/handbook/intro.html",
];
const JAVA_REFS: &[&str] = &[
    "https://dev.java/learn/",
    "https://docs.oracle.com/javase/tutorial/",
];
const RUST_REFS: &[&str] = &["https://doc.rust-lang.org/book/"];

const EMPTY_HELLO: &[SyntaxCase] = &[SyntaxCase {
    input: "",
    output: "ok\n",
}];
const ECHO_CASE: &[SyntaxCase] = &[SyntaxCase {
    input: "code\n",
    output: "code\n",
}];
const SUM_CASE: &[SyntaxCase] = &[SyntaxCase {
    input: "2 3\n",
    output: "5\n",
}];

const PYTHON_LESSONS: &[SyntaxLesson] = &[
    lesson!(
        "py-output",
        "python",
        "basic",
        "Output",
        "Use print for visible output.",
        "print('ok')",
        "# TODO: print exactly ok\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-variables",
        "python",
        "basic",
        "Variables",
        "Names bind to values and can be rebound.",
        "count = 1\nprint(count)",
        "word = None\n# TODO: bind word to 'ok', then print word\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-strings",
        "python",
        "basic",
        "Strings",
        "Strings support len, indexing, slicing, and iteration.",
        "text = 'code'\nprint(text[:2])",
        "text = 'xokx'\n# TODO: use a slice to print ok\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-control-flow",
        "python",
        "basic",
        "Control flow",
        "Use if, for, and while to choose and repeat work.",
        "for n in range(3):\n    print(n)",
        "ready = True\n# TODO: print ok only when ready is true\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-functions",
        "python",
        "basic",
        "Functions",
        "def creates reusable behavior with parameters and returns.",
        "def add(a, b):\n    return a + b",
        "def word():\n    # TODO: return ok as a string\n    pass\n\nprint(word())\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-input",
        "python",
        "intermediate",
        "Input parsing",
        "sys.stdin plus split handles contest-style input.",
        "import sys\nnums = list(map(int, sys.stdin.read().split()))",
        "import sys\ntext = sys.stdin.read()\n# TODO: write text back unchanged\n",
        ECHO_CASE,
        PY_REFS
    ),
    lesson!(
        "py-lists-dicts",
        "python",
        "intermediate",
        "Lists and dicts",
        "Lists keep order; dicts map keys to values.",
        "counts = {'a': 2}\nprint(counts['a'])",
        "nums = [2, 3]\n# TODO: print the sum of nums without hard-coding 5\n",
        SUM_CASE,
        PY_REFS
    ),
    lesson!(
        "py-errors",
        "python",
        "intermediate",
        "Exceptions",
        "try and except handle recoverable failures.",
        "try:\n    int('x')\nexcept ValueError:\n    print('bad')",
        "try:\n    int('x')\nexcept ValueError:\n    # TODO: handle the expected error by printing ok\n    pass\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-comprehensions",
        "python",
        "advanced",
        "Comprehensions",
        "Comprehensions build collections from expressions.",
        "evens = [n for n in range(5) if n % 2 == 0]",
        "letters = ['o', 'k']\n# TODO: build a word with a comprehension and print it\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-generators",
        "python",
        "advanced",
        "Iterators and generators",
        "yield creates lazy sequences.",
        "def ones():\n    yield 1",
        "def words():\n    # TODO: yield ok as a string\n    return\n\nprint(next(words()))\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-decorators",
        "python",
        "advanced",
        "Decorators",
        "Decorators wrap functions at definition time.",
        "def deco(fn):\n    return fn",
        "def deco(fn):\n    # TODO: return fn unchanged\n    pass\n\n@deco\ndef word():\n    return 'ok'\n\nprint(word())\n",
        EMPTY_HELLO,
        PY_REFS
    ),
    lesson!(
        "py-context-types",
        "python",
        "advanced",
        "Context managers and type hints",
        "with manages scoped resources; annotations document expected types.",
        "from typing import Iterable\n\ndef total(xs: Iterable[int]) -> int:\n    return sum(xs)",
        "from typing import Final\nword: Final[str] = 'ok'\n# TODO: print word\n",
        EMPTY_HELLO,
        &["https://docs.python.org/3/library/contextlib.html"]
    ),
];

const TS_LESSONS: &[SyntaxLesson] = &[
    lesson!(
        "ts-output",
        "ts",
        "basic",
        "Output",
        "console.log writes a line.",
        "console.log('ok');",
        "console.log('ok');\n",
        EMPTY_HELLO,
        TS_REFS
    ),
    lesson!(
        "ts-variables",
        "ts",
        "basic",
        "Variables",
        "let changes; const does not reassign.",
        "const name: string = 'code';",
        "const word: string = 'ok';\nconsole.log(word);\n",
        EMPTY_HELLO,
        TS_REFS
    ),
    lesson!(
        "ts-strings",
        "ts",
        "basic",
        "Strings",
        "Strings expose length and iteration.",
        "for (const ch of 'ok') console.log(ch);",
        "console.log('ok'.slice(0, 2));\n",
        EMPTY_HELLO,
        TS_REFS
    ),
    lesson!(
        "ts-control-flow",
        "ts",
        "basic",
        "Control flow",
        "if and loops control execution.",
        "for (let i = 0; i < 3; i++) {}",
        "if (true) console.log('ok');\n",
        EMPTY_HELLO,
        TS_REFS
    ),
    lesson!(
        "ts-functions",
        "ts",
        "basic",
        "Functions",
        "Parameter and return types make intent explicit.",
        "function add(a: number, b: number): number { return a + b; }",
        "function word(): string { return 'ok'; }\nconsole.log(word());\n",
        EMPTY_HELLO,
        TS_REFS
    ),
    lesson!(
        "ts-input",
        "ts",
        "intermediate",
        "Input parsing",
        "Node can read stdin for small exercises.",
        "const input = require('fs').readFileSync(0, 'utf8');",
        "const fs = require('fs');\nprocess.stdout.write(fs.readFileSync(0, 'utf8'));\n",
        ECHO_CASE,
        TS_REFS
    ),
    lesson!(
        "ts-arrays-objects",
        "ts",
        "intermediate",
        "Arrays and objects",
        "Arrays hold sequences; object types describe shapes.",
        "type User = { name: string };",
        "const nums: number[] = [2, 3];\nconsole.log(nums.reduce((a, b) => a + b, 0));\n",
        SUM_CASE,
        TS_REFS
    ),
    lesson!(
        "ts-errors-async",
        "ts",
        "intermediate",
        "Errors and async",
        "try/catch handles thrown errors; async wraps promises.",
        "async function main() { return 1; }",
        "try { throw new Error('x'); } catch { console.log('ok'); }\n",
        EMPTY_HELLO,
        &["https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await"]
    ),
    lesson!(
        "ts-narrowing",
        "ts",
        "advanced",
        "Narrowing",
        "Type guards refine union values.",
        "if (typeof value === 'string') value.toUpperCase();",
        "const value: string | number = 'ok';\nif (typeof value === 'string') console.log(value);\n",
        EMPTY_HELLO,
        TS_REFS
    ),
    lesson!(
        "ts-generics",
        "ts",
        "advanced",
        "Generics",
        "Generics preserve type information across reusable code.",
        "function first<T>(xs: T[]): T { return xs[0]; }",
        "function id<T>(value: T): T { return value; }\nconsole.log(id('ok'));\n",
        EMPTY_HELLO,
        TS_REFS
    ),
    lesson!(
        "ts-mapped",
        "ts",
        "advanced",
        "Mapped types",
        "Mapped types transform object properties.",
        "type ReadonlyUser<T> = { readonly [K in keyof T]: T[K] };",
        "type Box<T> = { [K in keyof T]: T[K] };\nconst value: Box<{ word: string }> = { word: 'ok' };\nconsole.log(value.word);\n",
        EMPTY_HELLO,
        &["https://www.typescriptlang.org/docs/handbook/utility-types.html"]
    ),
    lesson!(
        "ts-conditional",
        "ts",
        "advanced",
        "Conditional types",
        "Conditional types choose a type from another type.",
        "type Unwrap<T> = T extends Promise<infer U> ? U : T;",
        "type IsString<T> = T extends string ? string : never;\nconst word: IsString<'ok'> = 'ok';\nconsole.log(word);\n",
        EMPTY_HELLO,
        TS_REFS
    ),
];

const JAVA_LESSONS: &[SyntaxLesson] = &[
    lesson!(
        "java-output",
        "java",
        "basic",
        "Output",
        "System.out.println writes a line.",
        "System.out.println(\"ok\");",
        "class Solution { public static void main(String[] args) { System.out.println(\"ok\"); } }\n",
        EMPTY_HELLO,
        JAVA_REFS
    ),
    lesson!(
        "java-variables",
        "java",
        "basic",
        "Variables",
        "Java variables have declared types.",
        "String word = \"ok\";",
        "class Solution { public static void main(String[] args) { String word = \"ok\"; System.out.println(word); } }\n",
        EMPTY_HELLO,
        JAVA_REFS
    ),
    lesson!(
        "java-strings",
        "java",
        "basic",
        "Strings",
        "String methods expose length, chars, and substrings.",
        "\"code\".substring(0, 2)",
        "class Solution { public static void main(String[] args) { System.out.println(\"ok\".substring(0, 2)); } }\n",
        EMPTY_HELLO,
        JAVA_REFS
    ),
    lesson!(
        "java-control-flow",
        "java",
        "basic",
        "Control flow",
        "if, for, and while control execution.",
        "for (int i = 0; i < 3; i++) {}",
        "class Solution { public static void main(String[] args) { if (true) System.out.println(\"ok\"); } }\n",
        EMPTY_HELLO,
        JAVA_REFS
    ),
    lesson!(
        "java-methods",
        "java",
        "basic",
        "Methods",
        "Methods group reusable behavior.",
        "static int add(int a, int b) { return a + b; }",
        "class Solution { static String word() { return \"ok\"; } public static void main(String[] args) { System.out.println(word()); } }\n",
        EMPTY_HELLO,
        JAVA_REFS
    ),
    lesson!(
        "java-input",
        "java",
        "intermediate",
        "Input parsing",
        "System.in can be read directly for exercises.",
        "String input = new String(System.in.readAllBytes());",
        "import java.io.*;\nclass Solution { public static void main(String[] args) throws Exception { System.out.print(new String(System.in.readAllBytes())); } }\n",
        ECHO_CASE,
        JAVA_REFS
    ),
    lesson!(
        "java-arrays-collections",
        "java",
        "intermediate",
        "Arrays and collections",
        "Arrays are fixed size; collections add flexible containers.",
        "int[] nums = {1, 2};",
        "class Solution { public static void main(String[] args) { int[] nums = {2, 3}; System.out.println(nums[0] + nums[1]); } }\n",
        SUM_CASE,
        JAVA_REFS
    ),
    lesson!(
        "java-exceptions",
        "java",
        "intermediate",
        "Exceptions",
        "try/catch handles failures; checked exceptions are part of signatures.",
        "try { throw new RuntimeException(); } catch (RuntimeException e) {}",
        "class Solution { public static void main(String[] args) { try { throw new RuntimeException(); } catch (RuntimeException e) { System.out.println(\"ok\"); } } }\n",
        EMPTY_HELLO,
        JAVA_REFS
    ),
    lesson!(
        "java-classes-interfaces",
        "java",
        "advanced",
        "Classes and interfaces",
        "Classes hold state and behavior; interfaces describe behavior.",
        "interface Named { String name(); }",
        "interface Named { String name(); }\nclass Solution implements Named { public String name() { return \"ok\"; } public static void main(String[] args) { System.out.println(new Solution().name()); } }\n",
        EMPTY_HELLO,
        JAVA_REFS
    ),
    lesson!(
        "java-generics",
        "java",
        "advanced",
        "Generics",
        "Generics reuse code with type parameters.",
        "class Box<T> { T value; }",
        "class Box<T> { T value; Box(T value) { this.value = value; } }\nclass Solution { public static void main(String[] args) { System.out.println(new Box<String>(\"ok\").value); } }\n",
        EMPTY_HELLO,
        &["https://dev.java/learn/generics/"]
    ),
    lesson!(
        "java-lambda-streams",
        "java",
        "advanced",
        "Lambda and streams",
        "Lambdas pass behavior; streams process sequences.",
        "list.stream().map(x -> x + 1)",
        "import java.util.*;\nclass Solution { public static void main(String[] args) { List<String> xs = List.of(\"ok\"); xs.stream().forEach(System.out::println); } }\n",
        EMPTY_HELLO,
        &["https://docs.oracle.com/javase/tutorial/java/javaOO/lambdaexpressions.html"]
    ),
    lesson!(
        "java-records-sealed",
        "java",
        "advanced",
        "Records and sealed types",
        "Records reduce data boilerplate; sealed types bound inheritance.",
        "record Pair(int a, int b) {}",
        "record Word(String value) {}\nclass Solution { public static void main(String[] args) { System.out.println(new Word(\"ok\").value()); } }\n",
        EMPTY_HELLO,
        JAVA_REFS
    ),
];

const RUST_LESSONS: &[SyntaxLesson] = &[
    lesson!(
        "rust-output",
        "rust",
        "basic",
        "Output",
        "println! writes a line.",
        "println!(\"ok\");",
        "fn main() {\n    println!(\"ok\");\n}\n",
        EMPTY_HELLO,
        RUST_REFS
    ),
    lesson!(
        "rust-variables",
        "rust",
        "basic",
        "Variables",
        "let binds values; mut allows mutation.",
        "let mut count = 0;",
        "fn main() {\n    let word = \"ok\";\n    println!(\"{word}\");\n}\n",
        EMPTY_HELLO,
        RUST_REFS
    ),
    lesson!(
        "rust-strings",
        "rust",
        "basic",
        "Strings",
        "String owns text; &str borrows text.",
        "let s = String::from(\"ok\");",
        "fn main() {\n    let word = String::from(\"ok\");\n    println!(\"{word}\");\n}\n",
        EMPTY_HELLO,
        RUST_REFS
    ),
    lesson!(
        "rust-control-flow",
        "rust",
        "basic",
        "Control flow",
        "if is an expression; for iterates ranges and collections.",
        "for n in 0..3 {}",
        "fn main() {\n    if true { println!(\"ok\"); }\n}\n",
        EMPTY_HELLO,
        RUST_REFS
    ),
    lesson!(
        "rust-functions",
        "rust",
        "basic",
        "Functions",
        "Functions declare parameter and return types.",
        "fn add(a: i32, b: i32) -> i32 { a + b }",
        "fn word() -> &'static str { \"ok\" }\nfn main() { println!(\"{}\", word()); }\n",
        EMPTY_HELLO,
        RUST_REFS
    ),
    lesson!(
        "rust-input",
        "rust",
        "intermediate",
        "Input parsing",
        "Read stdin into a String, then split or print it.",
        "std::io::stdin().read_to_string(&mut input)",
        "use std::io::{self, Read};\nfn main() {\n    let mut input = String::new();\n    io::stdin().read_to_string(&mut input).unwrap();\n    print!(\"{input}\");\n}\n",
        ECHO_CASE,
        RUST_REFS
    ),
    lesson!(
        "rust-vec-hashmap",
        "rust",
        "intermediate",
        "Vec and HashMap",
        "Vec stores sequences; HashMap stores key/value pairs.",
        "let nums = vec![1, 2];",
        "fn main() {\n    let nums = vec![2, 3];\n    println!(\"{}\", nums.iter().sum::<i32>());\n}\n",
        SUM_CASE,
        RUST_REFS
    ),
    lesson!(
        "rust-result",
        "rust",
        "intermediate",
        "Result and ?",
        "Result models recoverable errors; ? returns early on Err.",
        "fn parse() -> Result<i32, std::num::ParseIntError> { \"1\".parse() }",
        "fn word() -> Result<&'static str, ()> { Ok(\"ok\") }\nfn main() { println!(\"{}\", word().unwrap()); }\n",
        EMPTY_HELLO,
        RUST_REFS
    ),
    lesson!(
        "rust-ownership",
        "rust",
        "advanced",
        "Ownership and borrowing",
        "A value has one owner; references borrow without moving.",
        "fn len(s: &String) -> usize { s.len() }",
        "fn show(s: &str) { println!(\"{s}\"); }\nfn main() { show(\"ok\"); }\n",
        EMPTY_HELLO,
        &["https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html"]
    ),
    lesson!(
        "rust-enum-match",
        "rust",
        "advanced",
        "Enums and match",
        "Enums model alternatives; match handles them exhaustively.",
        "match value { Some(x) => x, None => 0 }",
        "fn main() {\n    let value = Some(\"ok\");\n    match value { Some(word) => println!(\"{word}\"), None => println!(\"no\") }\n}\n",
        EMPTY_HELLO,
        RUST_REFS
    ),
    lesson!(
        "rust-traits-lifetimes",
        "rust",
        "advanced",
        "Traits, generics, and lifetimes",
        "Traits define shared behavior; lifetimes describe borrowed relationships.",
        "fn first<'a>(x: &'a str) -> &'a str { x }",
        "fn id<'a>(value: &'a str) -> &'a str { value }\nfn main() { println!(\"{}\", id(\"ok\")); }\n",
        EMPTY_HELLO,
        &["https://doc.rust-lang.org/book/ch10-00-generics.html"]
    ),
    lesson!(
        "rust-iterators",
        "rust",
        "advanced",
        "Iterators and closures",
        "Iterators compose lazy transformations.",
        "nums.iter().map(|n| n + 1)",
        "fn main() {\n    let word = [\"o\", \"k\"].iter().copied().collect::<String>();\n    println!(\"{word}\");\n}\n",
        EMPTY_HELLO,
        RUST_REFS
    ),
];

pub fn syntax_lessons_for(language: &str) -> Vec<&'static SyntaxLesson> {
    let lessons = match normalize_language(language).as_str() {
        "ts" => TS_LESSONS,
        "java" => JAVA_LESSONS,
        "rust" => RUST_LESSONS,
        _ => PYTHON_LESSONS,
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
    format!(
        "# {}: {}\n\n{}: {}\n{}: {}\n{}: {done}/{total} ({completed})\n\n## {}\n\n{}\n\n## {}\n\n{}\n{}\n\n## {}\n\n{}\n\n## {}\n\n{}",
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
        extra_sections,
        ui_text(ui_language, "syntax_exercise"),
        localized_syntax_exercise_prompt(lesson, ui_language),
        ui_text(ui_language, "syntax_references"),
        refs
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
            "Worked example:\n{}\n\n```{}\n{}\n```",
            lesson_copy_for(lesson, ui_language)
                .map(|copy| copy.worked_example.as_str())
                .unwrap_or(""),
            lesson.language,
            lesson.example
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
    if let Some(copy) = lesson_copy_for(lesson, ui_language) {
        return copy.exercise_prompt.clone();
    }
    if normalize_ui_language(ui_language) == "en" {
        lesson.exercise.prompt.to_string()
    } else {
        ui_text(ui_language, "syntax_exercise_prompt").to_string()
    }
}

fn localized_syntax_title(lesson: &SyntaxLesson, ui_language: &str) -> String {
    lesson_copy_for(lesson, ui_language)
        .map(|copy| copy.title.clone())
        .or_else(|| localized_syntax_copy(lesson, ui_language, "title").map(str::to_string))
        .unwrap_or_else(|| lesson.title.to_string())
}

fn localized_syntax_body(lesson: &SyntaxLesson, ui_language: &str) -> String {
    lesson_copy_for(lesson, ui_language)
        .map(|copy| copy.concept.clone())
        .or_else(|| localized_syntax_copy(lesson, ui_language, "body").map(str::to_string))
        .unwrap_or_else(|| lesson.body.to_string())
}

fn localized_syntax_worked_example(lesson: &SyntaxLesson, ui_language: &str) -> String {
    let mut text = String::new();
    if let Some(copy) = lesson_copy_for(lesson, ui_language) {
        text.push_str(&copy.worked_example);
        text.push_str("\n\n");
    }
    text.push_str(&format!("```{}\n{}\n```", lesson.language, lesson.example));
    text
}

fn localized_syntax_list_section(
    lesson: &SyntaxLesson,
    ui_language: &str,
    title_key: &str,
    items: fn(&SyntaxLessonCopy) -> &Vec<String>,
) -> Option<String> {
    let copy = lesson_copy_for(lesson, ui_language)?;
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

fn localized_syntax_copy(
    lesson: &SyntaxLesson,
    ui_language: &str,
    field: &str,
) -> Option<&'static str> {
    let key = format!("syntax_{}_{}", lesson.id.replace('-', "_"), field);
    let copy = ui_text(ui_language, &key);
    if copy.is_empty() { None } else { Some(copy) }
}

fn lesson_copy_for(lesson: &SyntaxLesson, ui_language: &str) -> Option<&'static SyntaxLessonCopy> {
    if normalize_language(lesson.language) != "python" {
        return None;
    }
    let ui_language = normalize_ui_language(ui_language);
    let catalog = match ui_language.as_str() {
        "ko" => PYTHON_KO_COPY
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/python.ko.json"))),
        "en" => PYTHON_EN_COPY
            .get_or_init(|| load_lesson_copy(include_str!("../../assets/lessons/python.en.json"))),
        _ => return None,
    };
    catalog.get(lesson.id)
}

fn load_lesson_copy(text: &str) -> HashMap<String, SyntaxLessonCopy> {
    serde_json::from_str(text).expect("valid syntax lesson copy")
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

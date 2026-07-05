use super::*;

pub struct SyntaxLesson {
    pub id: &'static str,
    pub level: &'static str,
    pub titles: &'static [(&'static str, &'static str)],
    pub topics: &'static [&'static str],
    pub examples: &'static [SyntaxExample],
}

pub struct SyntaxExample {
    pub language: &'static str,
    pub code: &'static str,
}

const LESSONS: &[SyntaxLesson] = &[
    SyntaxLesson {
        id: "variables",
        level: "basic",
        titles: &[
            ("en", "Variables and types"),
            ("ko", "변수와 타입"),
            ("ja", "変数と型"),
            ("zh", "变量和类型"),
            ("es", "Variables y tipos"),
        ],
        topics: &[
            "variable",
            "variables",
            "type",
            "types",
            "number",
            "numbers",
        ],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "count = 3\nname = \"code\"\nprint(count, name)",
            },
            SyntaxExample {
                language: "ts",
                code: "let count: number = 3;\nconst name: string = \"code\";\nconsole.log(count, name);",
            },
            SyntaxExample {
                language: "java",
                code: "int count = 3;\nString name = \"code\";\nSystem.out.println(count + \" \" + name);",
            },
            SyntaxExample {
                language: "rust",
                code: "let count: i32 = 3;\nlet name = \"code\";\nprintln!(\"{count} {name}\");",
            },
        ],
    },
    SyntaxLesson {
        id: "io",
        level: "basic",
        titles: &[
            ("en", "Standard input/output"),
            ("ko", "표준 입출력"),
            ("ja", "標準入出力"),
            ("zh", "标准输入输出"),
            ("es", "Entrada/salida estandar"),
        ],
        topics: &["io", "stdin", "stdout", "input", "output"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "import sys\ntext = sys.stdin.read()\nprint(text, end=\"\")",
            },
            SyntaxExample {
                language: "ts",
                code: "const fs = require(\"fs\");\nconst input = fs.readFileSync(0, \"utf8\");\nprocess.stdout.write(input);",
            },
            SyntaxExample {
                language: "java",
                code: "import java.io.*;\n\nclass Solution {\n    public static void main(String[] args) throws Exception {\n        String input = new String(System.in.readAllBytes());\n        System.out.print(input);\n    }\n}",
            },
            SyntaxExample {
                language: "rust",
                code: "use std::io::{self, Read};\n\nfn main() {\n    let mut input = String::new();\n    io::stdin().read_to_string(&mut input).unwrap();\n    print!(\"{input}\");\n}",
            },
        ],
    },
    SyntaxLesson {
        id: "strings",
        level: "basic",
        titles: &[
            ("en", "Strings"),
            ("ko", "문자열"),
            ("ja", "文字列"),
            ("zh", "字符串"),
            ("es", "Cadenas"),
        ],
        topics: &["string", "strings", "char", "chars", "text"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "s = \"hello\"\nprint(len(s))\nfor ch in s:\n    print(ch)",
            },
            SyntaxExample {
                language: "ts",
                code: "const s = \"hello\";\nconsole.log(s.length);\nfor (const ch of s) {\n  console.log(ch);\n}",
            },
            SyntaxExample {
                language: "java",
                code: "String s = \"hello\";\nSystem.out.println(s.length());\nfor (int i = 0; i < s.length(); i++) {\n    System.out.println(s.charAt(i));\n}",
            },
            SyntaxExample {
                language: "rust",
                code: "let s = \"hello\";\nprintln!(\"{}\", s.chars().count());\nfor ch in s.chars() {\n    println!(\"{ch}\");\n}",
            },
        ],
    },
    SyntaxLesson {
        id: "loops",
        level: "basic",
        titles: &[
            ("en", "Loops and conditions"),
            ("ko", "반복문과 조건문"),
            ("ja", "ループと条件分岐"),
            ("zh", "循环和条件"),
            ("es", "Bucles y condiciones"),
        ],
        topics: &["loop", "loops", "condition", "conditions", "control-flow"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "for n in range(5):\n    if n % 2 == 0:\n        print(n)",
            },
            SyntaxExample {
                language: "ts",
                code: "for (let n = 0; n < 5; n++) {\n  if (n % 2 === 0) console.log(n);\n}",
            },
            SyntaxExample {
                language: "java",
                code: "for (int n = 0; n < 5; n++) {\n    if (n % 2 == 0) System.out.println(n);\n}",
            },
            SyntaxExample {
                language: "rust",
                code: "for n in 0..5 {\n    if n % 2 == 0 {\n        println!(\"{n}\");\n    }\n}",
            },
        ],
    },
    SyntaxLesson {
        id: "arrays",
        level: "basic",
        titles: &[
            ("en", "Arrays and lists"),
            ("ko", "배열과 리스트"),
            ("ja", "配列とリスト"),
            ("zh", "数组和列表"),
            ("es", "Arreglos y listas"),
        ],
        topics: &["array", "arrays", "list", "lists", "vec", "vector"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "nums = [1, 2, 3]\nnums.append(4)\nprint(sum(nums))",
            },
            SyntaxExample {
                language: "ts",
                code: "const nums = [1, 2, 3];\nnums.push(4);\nconsole.log(nums.reduce((a, b) => a + b, 0));",
            },
            SyntaxExample {
                language: "java",
                code: "int[] nums = {1, 2, 3};\nint sum = 0;\nfor (int n : nums) sum += n;\nSystem.out.println(sum);",
            },
            SyntaxExample {
                language: "rust",
                code: "let nums = vec![1, 2, 3];\nlet sum: i32 = nums.iter().sum();\nprintln!(\"{sum}\");",
            },
        ],
    },
    SyntaxLesson {
        id: "parsing",
        level: "intermediate",
        titles: &[
            ("en", "Parsing tokens"),
            ("ko", "토큰 파싱"),
            ("ja", "トークンの解析"),
            ("zh", "解析标记"),
            ("es", "Parseo de tokens"),
        ],
        topics: &["parse", "parsing", "token", "tokens", "split", "scanner"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "nums = list(map(int, input().split()))\nprint(sum(nums))",
            },
            SyntaxExample {
                language: "ts",
                code: "const nums = input.trim().split(/\\s+/).map(Number);\nconsole.log(nums.reduce((a, b) => a + b, 0));",
            },
            SyntaxExample {
                language: "java",
                code: "String[] parts = input.trim().split(\"\\\\s+\");\nint sum = 0;\nfor (String part : parts) sum += Integer.parseInt(part);\nSystem.out.println(sum);",
            },
            SyntaxExample {
                language: "rust",
                code: "let nums: Vec<i32> = input.split_whitespace()\n    .map(|s| s.parse().unwrap())\n    .collect();\nprintln!(\"{}\", nums.iter().sum::<i32>());",
            },
        ],
    },
    SyntaxLesson {
        id: "maps",
        level: "intermediate",
        titles: &[
            ("en", "Maps and dictionaries"),
            ("ko", "맵과 딕셔너리"),
            ("ja", "マップと辞書"),
            ("zh", "映射和字典"),
            ("es", "Mapas y diccionarios"),
        ],
        topics: &["map", "maps", "dict", "dictionary", "hashmap", "hash-map"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "counts = {}\nfor ch in \"banana\":\n    counts[ch] = counts.get(ch, 0) + 1\nprint(counts[\"a\"])",
            },
            SyntaxExample {
                language: "ts",
                code: "const counts = new Map<string, number>();\nfor (const ch of \"banana\") {\n  counts.set(ch, (counts.get(ch) ?? 0) + 1);\n}\nconsole.log(counts.get(\"a\"));",
            },
            SyntaxExample {
                language: "java",
                code: "Map<Character, Integer> counts = new HashMap<>();\nfor (char ch : \"banana\".toCharArray()) {\n    counts.put(ch, counts.getOrDefault(ch, 0) + 1);\n}\nSystem.out.println(counts.get('a'));",
            },
            SyntaxExample {
                language: "rust",
                code: "use std::collections::HashMap;\nlet mut counts = HashMap::new();\nfor ch in \"banana\".chars() {\n    *counts.entry(ch).or_insert(0) += 1;\n}\nprintln!(\"{}\", counts[&'a']);",
            },
        ],
    },
    SyntaxLesson {
        id: "sorting",
        level: "intermediate",
        titles: &[
            ("en", "Sorting"),
            ("ko", "정렬"),
            ("ja", "ソート"),
            ("zh", "排序"),
            ("es", "Ordenacion"),
        ],
        topics: &[
            "sort",
            "sorting",
            "order",
            "ordering",
            "compare",
            "comparator",
        ],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "items = [(\"b\", 2), (\"a\", 3)]\nitems.sort(key=lambda item: item[0])\nprint(items)",
            },
            SyntaxExample {
                language: "ts",
                code: "const items: [string, number][] = [[\"b\", 2], [\"a\", 3]];\nitems.sort((a, b) => a[0].localeCompare(b[0]));\nconsole.log(items);",
            },
            SyntaxExample {
                language: "java",
                code: "List<String> items = new ArrayList<>(List.of(\"b:2\", \"a:3\"));\nitems.sort(Comparator.naturalOrder());\nSystem.out.println(items);",
            },
            SyntaxExample {
                language: "rust",
                code: "let mut items = vec![(\"b\", 2), (\"a\", 3)];\nitems.sort_by_key(|item| item.0);\nprintln!(\"{:?}\", items);",
            },
        ],
    },
    SyntaxLesson {
        id: "classes",
        level: "intermediate",
        titles: &[
            ("en", "Structs and classes"),
            ("ko", "구조체와 클래스"),
            ("ja", "構造体とクラス"),
            ("zh", "结构体和类"),
            ("es", "Structs y clases"),
        ],
        topics: &["class", "classes", "struct", "structs", "object", "objects"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "class Point:\n    def __init__(self, x, y):\n        self.x = x\n        self.y = y\n\np = Point(1, 2)\nprint(p.x + p.y)",
            },
            SyntaxExample {
                language: "ts",
                code: "type Point = { x: number; y: number };\nconst p: Point = { x: 1, y: 2 };\nconsole.log(p.x + p.y);",
            },
            SyntaxExample {
                language: "java",
                code: "record Point(int x, int y) {}\nPoint p = new Point(1, 2);\nSystem.out.println(p.x() + p.y());",
            },
            SyntaxExample {
                language: "rust",
                code: "struct Point { x: i32, y: i32 }\nlet p = Point { x: 1, y: 2 };\nprintln!(\"{}\", p.x + p.y);",
            },
        ],
    },
    SyntaxLesson {
        id: "functions",
        level: "intermediate",
        titles: &[
            ("en", "Functions"),
            ("ko", "함수"),
            ("ja", "関数"),
            ("zh", "函数"),
            ("es", "Funciones"),
        ],
        topics: &["function", "functions", "method", "methods"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "def add(a, b):\n    return a + b\n\nprint(add(2, 3))",
            },
            SyntaxExample {
                language: "ts",
                code: "function add(a: number, b: number): number {\n  return a + b;\n}\n\nconsole.log(add(2, 3));",
            },
            SyntaxExample {
                language: "java",
                code: "static int add(int a, int b) {\n    return a + b;\n}\n\nSystem.out.println(add(2, 3));",
            },
            SyntaxExample {
                language: "rust",
                code: "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nprintln!(\"{}\", add(2, 3));",
            },
        ],
    },
    SyntaxLesson {
        id: "iterators",
        level: "advanced",
        titles: &[
            ("en", "Iterators and pipelines"),
            ("ko", "이터레이터와 파이프라인"),
            ("ja", "イテレータとパイプライン"),
            ("zh", "迭代器和管道"),
            ("es", "Iteradores y tuberias"),
        ],
        topics: &[
            "iterator",
            "iterators",
            "stream",
            "streams",
            "pipeline",
            "comprehension",
        ],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "squares = [n * n for n in range(10) if n % 2 == 0]\nprint(sum(squares))",
            },
            SyntaxExample {
                language: "ts",
                code: "const total = [...Array(10).keys()]\n  .filter(n => n % 2 === 0)\n  .map(n => n * n)\n  .reduce((a, b) => a + b, 0);\nconsole.log(total);",
            },
            SyntaxExample {
                language: "java",
                code: "int total = IntStream.range(0, 10)\n    .filter(n -> n % 2 == 0)\n    .map(n -> n * n)\n    .sum();\nSystem.out.println(total);",
            },
            SyntaxExample {
                language: "rust",
                code: "let total: i32 = (0..10)\n    .filter(|n| n % 2 == 0)\n    .map(|n| n * n)\n    .sum();\nprintln!(\"{total}\");",
            },
        ],
    },
    SyntaxLesson {
        id: "pattern-matching",
        level: "advanced",
        titles: &[
            ("en", "Pattern matching"),
            ("ko", "패턴 매칭"),
            ("ja", "パターンマッチ"),
            ("zh", "模式匹配"),
            ("es", "Pattern matching"),
        ],
        topics: &["match", "pattern", "patterns", "switch", "case"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "match value:\n    case 0:\n        print(\"zero\")\n    case _:\n        print(\"other\")",
            },
            SyntaxExample {
                language: "ts",
                code: "switch (value) {\n  case 0:\n    console.log(\"zero\");\n    break;\n  default:\n    console.log(\"other\");\n}",
            },
            SyntaxExample {
                language: "java",
                code: "switch (value) {\n    case 0 -> System.out.println(\"zero\");\n    default -> System.out.println(\"other\");\n}",
            },
            SyntaxExample {
                language: "rust",
                code: "match value {\n    0 => println!(\"zero\"),\n    _ => println!(\"other\"),\n}",
            },
        ],
    },
    SyntaxLesson {
        id: "generics",
        level: "advanced",
        titles: &[
            ("en", "Generics"),
            ("ko", "제네릭"),
            ("ja", "ジェネリクス"),
            ("zh", "泛型"),
            ("es", "Genericos"),
        ],
        topics: &["generic", "generics", "type-parameter", "template"],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "from typing import TypeVar\nT = TypeVar(\"T\")\n\ndef first(items: list[T]) -> T:\n    return items[0]",
            },
            SyntaxExample {
                language: "ts",
                code: "function first<T>(items: T[]): T {\n  return items[0];\n}",
            },
            SyntaxExample {
                language: "java",
                code: "static <T> T first(List<T> items) {\n    return items.get(0);\n}",
            },
            SyntaxExample {
                language: "rust",
                code: "fn first<T>(items: &[T]) -> &T {\n    &items[0]\n}",
            },
        ],
    },
    SyntaxLesson {
        id: "errors",
        level: "advanced",
        titles: &[
            ("en", "Errors and results"),
            ("ko", "오류와 결과 처리"),
            ("ja", "エラーと結果の処理"),
            ("zh", "错误和结果处理"),
            ("es", "Errores y resultados"),
        ],
        topics: &[
            "error",
            "errors",
            "exception",
            "exceptions",
            "result",
            "optional",
        ],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "try:\n    value = int(text)\nexcept ValueError:\n    value = 0\nprint(value)",
            },
            SyntaxExample {
                language: "ts",
                code: "const value = Number.parseInt(text, 10);\nif (Number.isNaN(value)) {\n  throw new Error(\"not a number\");\n}\nconsole.log(value);",
            },
            SyntaxExample {
                language: "java",
                code: "try {\n    int value = Integer.parseInt(text);\n    System.out.println(value);\n} catch (NumberFormatException error) {\n    System.out.println(0);\n}",
            },
            SyntaxExample {
                language: "rust",
                code: "let value = text.parse::<i32>().unwrap_or(0);\nprintln!(\"{value}\");",
            },
        ],
    },
    SyntaxLesson {
        id: "lifetimes",
        level: "advanced",
        titles: &[
            ("en", "References and ownership"),
            ("ko", "참조와 소유권"),
            ("ja", "参照と所有権"),
            ("zh", "引用和所有权"),
            ("es", "Referencias y propiedad"),
        ],
        topics: &[
            "reference",
            "references",
            "ownership",
            "borrow",
            "borrowing",
            "lifetime",
        ],
        examples: &[
            SyntaxExample {
                language: "python",
                code: "items = [1, 2, 3]\nalias = items\nalias.append(4)\nprint(items)",
            },
            SyntaxExample {
                language: "ts",
                code: "const items = [1, 2, 3];\nconst alias = items;\nalias.push(4);\nconsole.log(items);",
            },
            SyntaxExample {
                language: "java",
                code: "List<Integer> items = new ArrayList<>(List.of(1, 2, 3));\nList<Integer> alias = items;\nalias.add(4);\nSystem.out.println(items);",
            },
            SyntaxExample {
                language: "rust",
                code: "let items = vec![1, 2, 3];\nlet borrowed = &items;\nprintln!(\"{}\", borrowed.len());",
            },
        ],
    },
];

pub fn syntax_lessons() -> &'static [SyntaxLesson] {
    LESSONS
}

pub fn syntax_lessons_for_problem(problem: &Problem) -> Vec<&'static SyntaxLesson> {
    LESSONS
        .iter()
        .filter(|lesson| {
            problem
                .topics
                .iter()
                .any(|topic| lesson.matches_topic(topic))
        })
        .collect()
}

pub fn syntax_code_for(lesson: &SyntaxLesson, language: &str) -> &'static str {
    let language = normalize_language(language);
    lesson
        .examples
        .iter()
        .find(|example| example.language == language)
        .or_else(|| lesson.examples.first())
        .map(|example| example.code)
        .unwrap_or("")
}

pub fn syntax_language_name(language: &str) -> &'static str {
    match normalize_language(language).as_str() {
        "python" => "Python",
        "ts" => "TypeScript",
        "java" => "Java",
        "rust" => "Rust",
        _ => "Python",
    }
}

pub fn syntax_lesson_title(lesson: &SyntaxLesson, ui_language: &str) -> &'static str {
    let lang = normalize_ui_language(ui_language);
    lesson
        .titles
        .iter()
        .find(|(key, _)| *key == lang)
        .or_else(|| lesson.titles.iter().find(|(key, _)| *key == "en"))
        .or_else(|| lesson.titles.first())
        .map(|(_, title)| *title)
        .unwrap_or("")
}

pub fn syntax_level_label(level: &str, ui_language: &str) -> &'static str {
    let key = match level {
        "intermediate" => "syntax_level_intermediate",
        "advanced" => "syntax_level_advanced",
        _ => "syntax_level_basic",
    };
    ui_text(ui_language, key)
}

pub fn syntax_lesson_completed(state: &AppState, language: &str, lesson_id: &str) -> bool {
    let language = normalize_language(language);
    state
        .syntax_progress
        .get(&language)
        .is_some_and(|ids| ids.iter().any(|id| id == lesson_id))
}

pub fn syntax_progress_count(state: &AppState, language: &str) -> (usize, usize) {
    let language = normalize_language(language);
    let done = state
        .syntax_progress
        .get(&language)
        .map(|ids| ids.len())
        .unwrap_or_default();
    (done, LESSONS.len())
}

pub fn record_syntax_progress(state: &mut AppState, problem: &Problem) {
    let lesson_ids = syntax_lessons_for_problem(problem)
        .into_iter()
        .map(|lesson| lesson.id.to_string())
        .collect::<Vec<_>>();
    if lesson_ids.is_empty() {
        return;
    }
    let language = normalize_language(&state.settings.language);
    let mut ids = state.syntax_progress.remove(&language).unwrap_or_default();
    for id in lesson_ids {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    state
        .syntax_progress
        .insert(language, normalize_lesson_ids(&ids));
}

pub fn normalize_syntax_progress(
    progress: &HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut normalized = HashMap::new();
    for (language, ids) in progress {
        let language = language.trim().to_lowercase();
        if LANGUAGES.contains(&language.as_str()) {
            let ids = normalize_lesson_ids(ids);
            if !ids.is_empty() {
                normalized.insert(language, ids);
            }
        }
    }
    normalized
}

pub fn syntax_lesson_text(
    problem: &Problem,
    language: &str,
    ui_language: &str,
    state: &AppState,
) -> String {
    let language = normalize_language(language);
    let lessons = syntax_lessons_for_problem(problem);
    let lang = normalize_ui_language(ui_language);
    if lessons.is_empty() {
        return ui_text(&lang, "syntax_no_lesson").to_string();
    }

    syntax_lesson_text_for_lessons(&lessons, &language, &lang, state)
}

pub fn syntax_curriculum_text(language: &str, ui_language: &str, state: &AppState) -> String {
    let language = normalize_language(language);
    let lang = normalize_ui_language(ui_language);
    let lessons = LESSONS.iter().collect::<Vec<_>>();
    syntax_lesson_text_for_lessons(&lessons, &language, &lang, state)
}

fn syntax_lesson_text_for_lessons(
    lessons: &[&'static SyntaxLesson],
    language: &str,
    ui_language: &str,
    state: &AppState,
) -> String {
    let name = syntax_language_name(language);
    let mut lines = vec![format!("# {}: {name}", ui_text(ui_language, "syntax"))];
    for lesson in lessons {
        let checked = if syntax_lesson_completed(state, language, lesson.id) {
            "[x]"
        } else {
            "[ ]"
        };
        lines.extend([
            String::new(),
            format!(
                "## {checked} [{}] {}",
                syntax_level_label(lesson.level, ui_language),
                syntax_lesson_title(lesson, ui_language)
            ),
            String::new(),
            format!("```{language}"),
            syntax_code_for(lesson, language).to_string(),
            "```".to_string(),
        ]);
    }
    lines.extend([
        String::new(),
        format!("{} ({})", ui_text(ui_language, "syntax_practice"), name),
    ]);
    lines.join("\n")
}

impl SyntaxLesson {
    fn matches_topic(&self, topic: &str) -> bool {
        let topic = topic.trim().to_lowercase();
        self.topics.contains(&topic.as_str())
    }
}

fn normalize_lesson_ids(ids: &[String]) -> Vec<String> {
    LESSONS
        .iter()
        .filter(|lesson| ids.iter().any(|id| id == lesson.id))
        .map(|lesson| lesson.id.to_string())
        .collect()
}

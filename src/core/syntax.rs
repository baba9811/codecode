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

const PY_CORE_REFS: &[&str] = &[
    "https://docs.python.org/3/tutorial/index.html",
    "https://docs.python.org/3/reference/index.html",
    "https://docs.python.org/3/library/index.html",
    "https://peps.python.org/pep-0008/",
];
const TS_REFS: &[&str] = &[
    "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide",
    "https://www.typescriptlang.org/docs/handbook/intro.html",
    "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html",
    "https://www.typescriptlang.org/docs/handbook/2/narrowing.html",
    "https://nodejs.org/api/typescript.html",
];
const TS_NODE_REFS: &[&str] = &[
    "https://nodejs.org/api/typescript.html",
    "https://nodejs.org/api/fs.html#fsreadfilesyncpath-options",
    "https://nodejs.org/api/process.html#processstdout",
];
const TS_ARRAY_REFS: &[&str] = &[
    "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map",
    "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter",
    "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce",
    "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Loops_and_iteration",
];
const TS_TYPE_REFS: &[&str] = &[
    "https://www.typescriptlang.org/docs/handbook/2/generics.html",
    "https://www.typescriptlang.org/docs/handbook/2/keyof-types.html",
    "https://www.typescriptlang.org/docs/handbook/2/typeof-types.html",
    "https://www.typescriptlang.org/docs/handbook/2/indexed-access-types.html",
    "https://www.typescriptlang.org/docs/handbook/2/mapped-types.html",
    "https://www.typescriptlang.org/docs/handbook/2/conditional-types.html",
    "https://www.typescriptlang.org/docs/handbook/utility-types.html",
    "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-0.html",
    "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-4.html",
    "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-6.html",
    "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-9.html",
];
const JAVA_CORE_REFS: &[&str] = &[
    "https://dev.java/learn/",
    "https://docs.oracle.com/javase/tutorial/",
    "https://docs.oracle.com/javase/specs/jls/se21/html/index.html",
];
const JAVA_LANGUAGE_REFS: &[&str] = &[
    "https://dev.java/learn/",
    "https://docs.oracle.com/javase/tutorial/java/nutsandbolts/index.html",
    "https://docs.oracle.com/javase/specs/jls/se21/html/index.html",
];
const JAVA_CLASS_REFS: &[&str] = &[
    "https://dev.java/learn/",
    "https://docs.oracle.com/javase/tutorial/java/javaOO/classes.html",
    "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
];
const JAVA_COLLECTION_REFS: &[&str] = &[
    "https://dev.java/learn/",
    "https://docs.oracle.com/javase/tutorial/collections/index.html",
    "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/List.html",
    "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/Map.html",
    "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/Set.html",
];
const JAVA_EXCEPTION_REFS: &[&str] = &[
    "https://dev.java/learn/",
    "https://docs.oracle.com/javase/tutorial/essential/exceptions/",
    "https://docs.oracle.com/javase/specs/jls/se21/html/jls-11.html",
    "https://docs.oracle.com/javase/specs/jls/se21/html/jls-14.html",
];
const JAVA_STREAM_REFS: &[&str] = &[
    "https://dev.java/learn/",
    "https://docs.oracle.com/javase/tutorial/java/javaOO/lambdaexpressions.html",
    "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/stream/Stream.html",
];
const EMPTY_HELLO: &[SyntaxCase] = &[SyntaxCase {
    input: "",
    output: "ok\n",
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
        "print and stdout",
        "print converts values to text, writes them to stdout, and adds a newline unless told otherwise.",
        r#"name = 'Ada'
score = 7
print('example:', f'{name}:{score}')"#,
        r#"name = 'Ada'
score = 7
# TODO: format the variables for stdout
print('TODO')
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:7\n",
        }],
        &[
            "https://docs.python.org/3/library/functions.html#print",
            "https://docs.python.org/3/tutorial/inputoutput.html",
            "https://peps.python.org/pep-0008/",
        ]
    ),
    lesson!(
        "py-variables",
        "python",
        "basic",
        "Variables",
        "Assignment binds a name to an object; rebinding changes what the name points at, not the old object.",
        r#"count = 1
count = count + 2
print('example:', count)"#,
        r#"word = 'todo'
# TODO: rebind word before it reaches stdout
print(word)
"#,
        EMPTY_HELLO,
        PY_CORE_REFS
    ),
    lesson!(
        "py-numbers",
        "python",
        "basic",
        "Numbers",
        "int and float cover most numeric work; //, %, and ** are common in problem solutions.",
        r#"total = 7
size = 2
print('example:', f'{total // size}:{total % size}')"#,
        r#"total = 7
size = 2
# TODO: use integer division and remainder together
print(f'{total / size}:0')
"#,
        &[SyntaxCase {
            input: "",
            output: "3:1\n",
        }],
        &[
            "https://docs.python.org/3/tutorial/introduction.html#numbers",
            "https://docs.python.org/3/library/stdtypes.html#numeric-types-int-float-complex",
        ]
    ),
    lesson!(
        "py-strings",
        "python",
        "basic",
        "Strings",
        "Strings are immutable sequences, so indexing and slicing read characters without changing the original text.",
        r#"text = 'python'
print('example:', text[1:4])"#,
        r#"text = 'xokx'
# TODO: slice away the marker characters before printing
print(text)
"#,
        EMPTY_HELLO,
        &[
            "https://docs.python.org/3/tutorial/introduction.html#text",
            "https://docs.python.org/3/library/stdtypes.html#text-sequence-type-str",
        ]
    ),
    lesson!(
        "py-control-flow",
        "python",
        "basic",
        "Control flow",
        "if chooses a block, for iterates over an iterable, and while repeats until its condition changes.",
        r#"total = 0
for n in range(1, 4):
    if n % 2 == 1:
        total += n
print('example:', total)"#,
        r#"total = 0
for n in range(1, 4):
    # TODO: add only odd numbers
    total += 0
print(total)
"#,
        &[SyntaxCase {
            input: "",
            output: "4\n",
        }],
        &[
            "https://docs.python.org/3/tutorial/controlflow.html",
            "https://docs.python.org/3/reference/compound_stmts.html",
        ]
    ),
    lesson!(
        "py-functions",
        "python",
        "basic",
        "Functions",
        "def creates a callable object; parameters receive arguments and return sends a value back to the caller.",
        r#"def area(width, height):
    return width * height

print('example:', area(3, 4))"#,
        r#"def area(width, height):
    # TODO: return rectangle area, not perimeter
    return width + height

print(area(3, 4))
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &[
            "https://docs.python.org/3/tutorial/controlflow.html#defining-functions",
            "https://docs.python.org/3/reference/compound_stmts.html#function-definitions",
        ]
    ),
    lesson!(
        "py-input",
        "python",
        "intermediate",
        "Input parsing",
        "stdin starts as text; read it once, split into tokens when structure matters, then convert tokens explicitly.",
        r#"import sys
nums = [int(token) for token in sys.stdin.read().split()]
print('example:', sum(nums))"#,
        r#"import sys
nums = []
# TODO: parse all integers from stdin and print their sum
print(sum(nums))
"#,
        SUM_CASE,
        &[
            "https://docs.python.org/3/library/sys.html#sys.stdin",
            "https://docs.python.org/3/tutorial/inputoutput.html",
        ]
    ),
    lesson!(
        "py-lists-dicts",
        "python",
        "intermediate",
        "Lists and dicts",
        "Lists keep ordered values by position; dicts map keys to values for direct lookup and counting.",
        r#"scores = {'Ada': [2, 3], 'Lin': [4]}
print('example:', sum(scores['Ada']))"#,
        r#"nums = [2, 3]
scores = {'Ada': nums}
# TODO: print the sum stored under Ada without hard-coding 5
print(len(scores['Ada']))
"#,
        SUM_CASE,
        &[
            "https://docs.python.org/3/tutorial/datastructures.html#more-on-lists",
            "https://docs.python.org/3/tutorial/datastructures.html#dictionaries",
            "https://docs.python.org/3/library/stdtypes.html#mapping-types-dict",
        ]
    ),
    lesson!(
        "py-tuples-sets",
        "python",
        "basic",
        "Tuples and sets",
        "Tuples group a fixed sequence of values; sets keep unique members and make membership checks cheap.",
        r#"pair = ('o', 'k')
seen = set(pair)
print('example:', ''.join(pair), len(seen))"#,
        r#"pair = ('o', 'k')
seen = set()
# TODO: build a set from the tuple and print ok 2
print(''.join(pair[:1]), len(seen))
"#,
        &[SyntaxCase {
            input: "",
            output: "ok 2\n",
        }],
        &[
            "https://docs.python.org/3/tutorial/datastructures.html#tuples-and-sequences",
            "https://docs.python.org/3/tutorial/datastructures.html#sets",
            "https://docs.python.org/3/library/stdtypes.html#set-types-set-frozenset",
        ]
    ),
    lesson!(
        "py-comprehensions",
        "python",
        "intermediate",
        "Comprehensions",
        "A comprehension combines an output expression, a loop, and optional filters into one collection-building expression.",
        r#"nums = [1, 2, 3, 4]
squares = [n * n for n in nums if n % 2 == 0]
print('example:', sum(squares))"#,
        r#"letters = ['o', 'x', 'k']
# TODO: keep only the letters needed for ok with a comprehension
word = ''.join([ch for ch in letters if ch != 'x' and ch != 'k'])
print(word)
"#,
        EMPTY_HELLO,
        &[
            "https://docs.python.org/3/tutorial/datastructures.html#list-comprehensions",
            "https://docs.python.org/3/tutorial/datastructures.html#dictionaries",
        ]
    ),
    lesson!(
        "py-errors",
        "python",
        "intermediate",
        "Exceptions",
        "try isolates code that may fail; except handles a specific recoverable error without hiding unrelated bugs.",
        r#"try:
    value = int('12')
except ValueError:
    value = 0
print('example:', value)"#,
        r#"try:
    value = int('bad')
except ValueError:
    # TODO: recover in the ValueError branch
    value = 0
print(value)
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &[
            "https://docs.python.org/3/tutorial/errors.html",
            "https://docs.python.org/3/reference/compound_stmts.html#the-try-statement",
        ]
    ),
    lesson!(
        "py-files-context",
        "python",
        "intermediate",
        "Files and context managers",
        "with enters a managed scope and calls cleanup automatically; file handles and contextlib helpers use this pattern.",
        r#"from io import StringIO

with StringIO('ok') as handle:
    text = handle.read()
print('example:', text)"#,
        r#"from io import StringIO

with StringIO('ok') as handle:
    # TODO: read from the managed handle before it closes
    text = ''
print(text)
"#,
        EMPTY_HELLO,
        &[
            "https://docs.python.org/3/tutorial/inputoutput.html#reading-and-writing-files",
            "https://docs.python.org/3/reference/compound_stmts.html#the-with-statement",
            "https://docs.python.org/3/library/contextlib.html",
        ]
    ),
    lesson!(
        "py-modules-imports",
        "python",
        "basic",
        "Modules and imports",
        "import binds a module or object name so code can reuse standard-library behavior instead of rewriting it.",
        r#"import math

print('example:', math.ceil(2.1))"#,
        r#"import math

# TODO: use the imported module to round upward
print(math.floor(2.1))
"#,
        &[SyntaxCase {
            input: "",
            output: "3\n",
        }],
        &[
            "https://docs.python.org/3/tutorial/modules.html",
            "https://docs.python.org/3/reference/import.html",
            "https://peps.python.org/pep-0008/#imports",
        ]
    ),
    lesson!(
        "py-dataclasses",
        "python",
        "intermediate",
        "Dataclasses",
        "dataclass generates the routine class methods for simple data containers while leaving behavior explicit.",
        r#"from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int

point = Point(2, 3)
print('example:', point.x + point.y)"#,
        r#"from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int

point = Point(2, 3)
# TODO: use both fields
print(point.x)
"#,
        SUM_CASE,
        &["https://docs.python.org/3/library/dataclasses.html"]
    ),
    lesson!(
        "py-typing",
        "python",
        "intermediate",
        "Type hints",
        "Type hints document expected shapes for readers and tools; Python still executes values dynamically at runtime.",
        r#"from typing import Iterable

def total(values: Iterable[int]) -> int:
    return sum(values)

print('example:', total([2, 3]))"#,
        r#"from typing import Iterable

def total(values: Iterable[int]) -> int:
    # TODO: return the sum of the iterable
    return 0

print(total([2, 3]))
"#,
        SUM_CASE,
        &[
            "https://docs.python.org/3/library/typing.html",
            "https://docs.python.org/3/tutorial/controlflow.html#function-annotations",
        ]
    ),
    lesson!(
        "py-generators",
        "python",
        "advanced",
        "Iterators and generators",
        "Iterators produce values one at a time; a generator function uses yield to pause and resume that production.",
        r#"def countdown(n):
    while n > 0:
        yield n
        n -= 1

print('example:', next(countdown(3)))"#,
        r#"def words():
    # TODO: yield ok as the first generated value
    yield ''

print(next(words()))
"#,
        EMPTY_HELLO,
        &[
            "https://docs.python.org/3/tutorial/classes.html#iterators",
            "https://docs.python.org/3/tutorial/classes.html#generators",
            "https://docs.python.org/3/reference/simple_stmts.html#the-yield-statement",
        ]
    ),
    lesson!(
        "py-lambdas-closures",
        "python",
        "advanced",
        "Lambdas and closures",
        "lambda makes a small expression function; a closure remembers names from the surrounding scope.",
        r#"def make_adder(delta):
    return lambda value: value + delta

add_two = make_adder(2)
print('example:', add_two(3))"#,
        r#"def make_suffix(suffix):
    # TODO: return a lambda that appends suffix to word
    return lambda word: word

add_ok = make_suffix('ok')
print(add_ok(''))
"#,
        EMPTY_HELLO,
        &[
            "https://docs.python.org/3/tutorial/controlflow.html#lambda-expressions",
            "https://docs.python.org/3/reference/expressions.html#lambda",
            "https://docs.python.org/3/tutorial/classes.html#python-scopes-and-namespaces",
        ]
    ),
    lesson!(
        "py-decorators",
        "python",
        "advanced",
        "Decorators",
        "A decorator receives a function at definition time and returns the function object that name should now refer to.",
        r#"def identity(fn):
    return fn

@identity
def word():
    return 'ok'

print('example:', word())"#,
        r#"def identity(fn):
    # TODO: return the original function unchanged
    return lambda: ''

@identity
def word():
    return 'ok'

print(word())
"#,
        EMPTY_HELLO,
        &[
            "https://docs.python.org/3/reference/compound_stmts.html#function-definitions",
            "https://docs.python.org/3/glossary.html#term-decorator",
        ]
    ),
    lesson!(
        "py-sorting-keys",
        "python",
        "intermediate",
        "Sorting and key functions",
        "sorted returns a new ordered list; key functions choose the value used for each comparison.",
        r#"users = [('Ada', 3), ('Lin', 5), ('Bo', 4)]
best = sorted(users, key=lambda item: item[1], reverse=True)[0]
print('example:', f'{best[0]}:{best[1]}')"#,
        r#"users = [('Ada', 3), ('Lin', 5), ('Bo', 4)]
# TODO: sort by score descending, not by name
best = sorted(users)[0]
print(f'{best[0]}:{best[1]}')
"#,
        &[SyntaxCase {
            input: "",
            output: "Lin:5\n",
        }],
        &[
            "https://docs.python.org/3/library/functions.html#sorted",
            "https://docs.python.org/3/howto/sorting.html",
        ]
    ),
    lesson!(
        "py-counter-defaultdict",
        "python",
        "intermediate",
        "Counter and defaultdict",
        "Counter counts hashable values directly; defaultdict creates missing collection values when grouping.",
        r#"from collections import Counter, defaultdict

words = ['red', 'blue', 'red']
counts = Counter(words)
groups = defaultdict(list)
for word in words:
    groups[word[0]].append(word)
print('example:', counts['red'], len(groups['r']))"#,
        r#"from collections import Counter, defaultdict

words = ['red', 'blue', 'red']
counts = Counter()
groups = defaultdict(list)
# TODO: count words and group them by first letter
print(counts['red'], len(groups['r']))
"#,
        &[SyntaxCase {
            input: "",
            output: "2 2\n",
        }],
        &["https://docs.python.org/3/library/collections.html"]
    ),
    lesson!(
        "py-deque",
        "python",
        "intermediate",
        "deque",
        "deque supports efficient appends and pops on both ends, which is why it is the usual queue type.",
        r#"from collections import deque

queue = deque(['middle'])
queue.appendleft('start')
queue.append('end')
print('example:', queue.popleft(), queue.pop())"#,
        r#"from collections import deque

queue = deque(['middle'])
# TODO: add start on the left and end on the right
print(queue.popleft(), 'missing')
"#,
        &[SyntaxCase {
            input: "",
            output: "start end\n",
        }],
        &["https://docs.python.org/3/library/collections.html#collections.deque"]
    ),
    lesson!(
        "py-itertools",
        "python",
        "advanced",
        "itertools",
        "itertools provides lazy iterator building blocks for pairing, chaining, slicing, and combinatorics.",
        r#"import itertools

parts = [['o'], ['k']]
print('example:', ''.join(itertools.chain.from_iterable(parts)))"#,
        r#"import itertools

parts = [['o'], ['k']]
# TODO: flatten both inner lists lazily
print(''.join(itertools.chain.from_iterable(parts[:1])))
"#,
        EMPTY_HELLO,
        &["https://docs.python.org/3/library/itertools.html"]
    ),
    lesson!(
        "py-pathlib",
        "python",
        "intermediate",
        "pathlib",
        "pathlib represents paths as objects, so code can ask for names, suffixes, and parents without manual string splitting.",
        r#"from pathlib import PurePosixPath

path = PurePosixPath('logs/app.txt')
print('example:', f'{path.stem}:{path.suffix}')"#,
        r#"from pathlib import PurePosixPath

path = PurePosixPath('logs/app.txt')
# TODO: combine the stem and suffix properties
print(path.name)
"#,
        &[SyntaxCase {
            input: "",
            output: "app:.txt\n",
        }],
        &["https://docs.python.org/3/library/pathlib.html"]
    ),
    lesson!(
        "py-testing-assert",
        "python",
        "intermediate",
        "Testing and assert",
        "assert checks an invariant in small examples; test frameworks build on the same idea with repeatable test functions.",
        r#"def add_two(value):
    return value + 2

assert add_two(3) == 5
print('example:', 'ok')"#,
        r#"def add_two(value):
    # TODO: make the assertion describe the intended behavior
    return value

assert add_two(3) == 3
print('todo')
"#,
        EMPTY_HELLO,
        &[
            "https://docs.python.org/3/reference/simple_stmts.html#the-assert-statement",
            "https://docs.python.org/3/library/unittest.html",
            "https://docs.python.org/3/tutorial/stdlib.html#quality-control",
        ]
    ),
    lesson!(
        "py-async",
        "python",
        "advanced",
        "Async concepts",
        "async def creates a coroutine; await pauses until the awaited operation completes, and asyncio.run drives the top-level coroutine.",
        r#"import asyncio

async def label():
    return 'ok'

async def main():
    print('example:', await label())

asyncio.run(main())"#,
        r#"import asyncio

async def label():
    return 'ok'

async def main():
    # TODO: await the coroutine before printing
    result = 'pending'
    print(result)

asyncio.run(main())
"#,
        EMPTY_HELLO,
        &[
            "https://docs.python.org/3/library/asyncio.html",
            "https://docs.python.org/3/reference/datamodel.html#coroutines",
            "https://docs.python.org/3/reference/expressions.html#await",
        ]
    ),
];

const TS_LESSONS: &[SyntaxLesson] = &[
    lesson!(
        "ts-output",
        "ts",
        "basic",
        "Console and stdout",
        "console.log appends a newline, while process.stdout.write writes exactly the bytes you give it.",
        r#"const score: number = 7;
process.stdout.write(`example:score=${score}\n`);"#,
        r#"const score: number = 7;
// TODO: format the score for stdout with one trailing newline
process.stdout.write('TODO\n');
"#,
        &[SyntaxCase {
            input: "",
            output: "score=7\n",
        }],
        TS_NODE_REFS
    ),
    lesson!(
        "ts-let-const",
        "ts",
        "basic",
        "let and const",
        "const protects a binding from reassignment; let marks the few local values that intentionally change.",
        r#"const label = 'sum';
let total = 1;
total += 2;
console.log('example:', `${label}:${total}`);"#,
        r#"const label = 'TODO';
let total = 1;
// TODO: keep label stable and mutate total before formatting
total += 0;
console.log(`${label}:${total}`);
"#,
        &[SyntaxCase {
            input: "",
            output: "sum:3\n",
        }],
        &[
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/let",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/const",
            "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html",
        ]
    ),
    lesson!(
        "ts-primitives",
        "ts",
        "basic",
        "Primitive types",
        "string, number, and boolean describe the common scalar values that most stdin parsing produces.",
        r#"function report(name: string, score: number, passed: boolean): string {
  return `${name}:${score}:${passed ? 'pass' : 'retry'}`;
}

console.log('example:', report('Ada', 7, true));"#,
        r#"function report(name: string, score: number, passed: boolean): string {
  return `${name}:${score}:${passed ? 'pass' : 'retry'}`;
}

// TODO: pass values whose types match the report contract
console.log(report('Ada', 0, false));
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:7:pass\n",
        }],
        TS_REFS
    ),
    lesson!(
        "ts-strings-templates",
        "ts",
        "basic",
        "Strings and templates",
        "Template literals keep formatting close to the values, and string methods return new strings instead of mutating text.",
        r#"const raw = ' Ada ';
const score = 7;
console.log('example:', `${raw.trim()}:${score}`);"#,
        r#"const raw = ' Ada ';
const score = 7;
// TODO: trim the name and interpolate score without changing either value
console.log(`${raw}:${score + 1}`);
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:7\n",
        }],
        &[
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String",
            "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html",
        ]
    ),
    lesson!(
        "ts-arrays-tuples",
        "ts",
        "basic",
        "Arrays and tuples",
        "number[] models a sequence of same-shaped values; a tuple fixes both position and type for small records.",
        r#"const scores: number[] = [2, 3];
const result: [string, number] = ['Ada', scores[0] + scores[1]];
console.log('example:', `${result[0]}:${result[1]}`);"#,
        r#"const scores: number[] = [2, 3];
// TODO: put the summed score in the tuple, not the array length
const result: [string, number] = ['Ada', scores.length];
console.log(`${result[0]}:${result[1]}`);
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:5\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#arrays",
            "https://www.typescriptlang.org/docs/handbook/2/objects.html#tuple-types",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array",
        ]
    ),
    lesson!(
        "ts-objects",
        "ts",
        "basic",
        "Object types",
        "Object type annotations name required fields so calculations cannot silently ignore missing properties.",
        r#"type Rectangle = { width: number; height: number };
const rect: Rectangle = { width: 3, height: 4 };
console.log('example:', rect.width * rect.height);"#,
        r#"type Rectangle = { width: number; height: number };
const rect: Rectangle = { width: 3, height: 4 };
// TODO: calculate area from both required fields
console.log(rect.width + rect.height);
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/objects.html",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Working_with_objects",
        ]
    ),
    lesson!(
        "ts-functions",
        "ts",
        "basic",
        "Functions",
        "Parameter and return annotations make the input and output contract visible at the call site.",
        r#"function area(width: number, height: number): number {
  return width * height;
}

console.log('example:', area(3, 4));"#,
        r#"function area(width: number, height: number): number {
  // TODO: return rectangle area, not perimeter
  return width + height;
}

console.log(area(3, 4));
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        TS_REFS
    ),
    lesson!(
        "ts-input",
        "ts",
        "intermediate",
        "Node stdin parsing",
        "In coding tests, read fd 0 once, split the text into tokens, and convert tokens before doing numeric work.",
        r#"const fs = require('node:fs');
const input: string = fs.readFileSync(0, 'utf8');
const nums = input.trim().split(/\s+/).filter(Boolean).map(Number);
console.log('example:', nums.reduce((sum, n) => sum + n, 0));"#,
        r#"const fs = require('node:fs');
const input: string = fs.readFileSync(0, 'utf8');
// TODO: parse all integers from stdin and print their sum
const nums: number[] = [];
console.log(nums.reduce((sum, n) => sum + n, 0));
"#,
        SUM_CASE,
        TS_NODE_REFS
    ),
    lesson!(
        "ts-control-flow",
        "ts",
        "basic",
        "Control flow",
        "if, for, while, and switch all narrow the path values can take before they reach stdout.",
        r#"let total = 0;
for (let n = 1; n <= 3; n++) {
  if (n % 2 === 1) total += n;
}
console.log('example:', total);"#,
        r#"let total = 0;
for (let n = 1; n <= 3; n++) {
  // TODO: add only odd numbers
  total += n;
}
console.log(total);
"#,
        &[SyntaxCase {
            input: "",
            output: "4\n",
        }],
        &[
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Control_flow_and_error_handling",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Loops_and_iteration",
            "https://www.typescriptlang.org/docs/handbook/2/narrowing.html",
        ]
    ),
    lesson!(
        "ts-union-narrowing",
        "ts",
        "intermediate",
        "Union and narrowing",
        "A union accepts several shapes, but TypeScript only allows member-specific operations after a runtime check narrows the value.",
        r#"function label(value: string | number): string {
  if (typeof value === 'string') return value.toUpperCase();
  return value.toFixed(0);
}

console.log('example:', label('ok'));"#,
        r#"function label(value: string | number): string {
  if (typeof value === 'string') return value;
  return value.toFixed(0);
}

// TODO: preserve the union but narrow the string branch to uppercase
console.log(label('ok'));
"#,
        &[SyntaxCase {
            input: "",
            output: "OK\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#union-types",
            "https://www.typescriptlang.org/docs/handbook/2/narrowing.html",
            "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-4.html",
        ]
    ),
    lesson!(
        "ts-literal-types",
        "ts",
        "intermediate",
        "Literal types",
        "Literal unions restrict values to exact strings or numbers, which is useful for modes, commands, and states.",
        r#"type Direction = 'left' | 'right';
function turn(direction: Direction): string {
  return direction === 'left' ? 'L' : 'R';
}

console.log('example:', turn('left'));"#,
        r#"type Direction = 'left' | 'right';
function turn(direction: Direction): string {
  return direction === 'left' ? 'L' : 'R';
}

// TODO: choose the allowed direction literal for the left branch
console.log(turn('right'));
"#,
        &[SyntaxCase {
            input: "",
            output: "L\n",
        }],
        TS_REFS
    ),
    lesson!(
        "ts-optional-nullish",
        "ts",
        "intermediate",
        "Optional and nullish",
        "Optional properties read as possibly undefined, and ?? keeps valid falsey values such as 0 or an empty string.",
        r#"type User = { name: string; score?: number | null };
const user: User = { name: 'Ada', score: 0 };
console.log('example:', `${user.name}:${user.score ?? 10}`);"#,
        r#"type User = { name: string; score?: number | null };
const user: User = { name: 'Ada', score: 0 };
// TODO: keep score 0 instead of replacing it with the fallback
console.log(`${user.name}:${user.score || 10}`);
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:0\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#optional-properties",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining",
        ]
    ),
    lesson!(
        "ts-interfaces-aliases",
        "ts",
        "intermediate",
        "Interfaces and type aliases",
        "Interfaces and aliases both name object contracts; aliases also name unions, tuples, and type expressions.",
        r#"interface Named {
  name: string;
}
type Score = { points: number };
function summary(user: Named & Score): string {
  return `${user.name}:${user.points}`;
}

console.log('example:', summary({ name: 'Ada', points: 5 }));"#,
        r#"interface Named {
  name: string;
}
type Score = { points: number };
function summary(user: Named & Score): string {
  // TODO: include both the interface field and the alias field
  return user.name;
}

console.log(summary({ name: 'Ada', points: 5 }));
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:5\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#interfaces",
            "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#type-aliases",
            "https://www.typescriptlang.org/docs/handbook/2/objects.html",
        ]
    ),
    lesson!(
        "ts-generics",
        "ts",
        "intermediate",
        "Generics",
        "Generics let reusable functions preserve the caller's type instead of collapsing values to any.",
        r#"function first<T>(items: readonly T[]): T | undefined {
  return items[0];
}

console.log('example:', first(['ok', 'skip']) ?? 'none');"#,
        r#"function first<T>(items: readonly T[]): T | undefined {
  // TODO: return the first item while preserving T
  return items[1];
}

console.log(first(['ok', 'skip']) ?? 'none');
"#,
        EMPTY_HELLO,
        TS_TYPE_REFS
    ),
    lesson!(
        "ts-keyof-typeof",
        "ts",
        "advanced",
        "keyof and typeof",
        "typeof captures the static type of a value, and keyof turns that object type into a union of valid keys.",
        r#"const limits = { small: 2, large: 5 } as const;
type Size = keyof typeof limits;
function limitFor(size: Size): number {
  return limits[size];
}

console.log('example:', limitFor('large'));"#,
        r#"const limits = { small: 2, large: 5 } as const;
type Size = keyof typeof limits;
function limitFor(size: Size): number {
  return limits[size];
}

// TODO: use the key that selects the larger limit
console.log(limitFor('small'));
"#,
        SUM_CASE,
        TS_TYPE_REFS
    ),
    lesson!(
        "ts-indexed-access",
        "ts",
        "advanced",
        "Indexed access types",
        "Indexed access types read a property type from another type so value code and type code stay in sync.",
        r#"type User = { name: string; scores: number[] };
type Score = User['scores'][number];
const score: Score = 5;
console.log('example:', score);"#,
        r#"type User = { name: string; scores: number[] };
type Score = User['scores'][number];
// TODO: assign a value that satisfies the indexed access type
const score: Score = 0;
console.log(score);
"#,
        SUM_CASE,
        TS_TYPE_REFS
    ),
    lesson!(
        "ts-mapped-types",
        "ts",
        "advanced",
        "Mapped types",
        "Mapped types loop over keys at the type level, often to turn one object shape into another related shape.",
        r#"type Flags<T> = { [K in keyof T]: boolean };
type Features = { search: () => void; share: () => void };
const enabled: Flags<Features> = { search: true, share: false };
console.log('example:', Object.entries(enabled).find(([, on]) => on)?.[0] ?? 'none');"#,
        r#"type Flags<T> = { [K in keyof T]: boolean };
type Features = { search: () => void; share: () => void };
const enabled: Flags<Features> = { search: false, share: false };
// TODO: enable search while keeping the mapped shape
console.log(Object.entries(enabled).find(([, on]) => on)?.[0] ?? 'none');
"#,
        EMPTY_HELLO,
        TS_TYPE_REFS
    ),
    lesson!(
        "ts-conditional-types",
        "ts",
        "advanced",
        "Conditional types",
        "Conditional types choose one type branch from another type, and infer can capture part of a matched shape.",
        r#"type ElementType<T> = T extends readonly (infer Item)[] ? Item : T;
const word: ElementType<string[]> = 'ok';
console.log('example:', word);"#,
        r#"type ElementType<T> = T extends readonly (infer Item)[] ? Item : T;
// TODO: assign the element type carried by string[]
const word: ElementType<string[]> = '';
console.log(word);
"#,
        EMPTY_HELLO,
        TS_TYPE_REFS
    ),
    lesson!(
        "ts-utility-types",
        "ts",
        "advanced",
        "Utility types",
        "Utility types such as Pick, Partial, Required, and Awaited express common transformations without custom aliases.",
        r#"type User = { id: number; name: string; score: number };
type UserPatch = Partial<Pick<User, 'name' | 'score'>>;
const patch: UserPatch = { name: 'Ada', score: 5 };
console.log('example:', `${patch.name}:${patch.score}`);"#,
        r#"type User = { id: number; name: string; score: number };
type UserPatch = Partial<Pick<User, 'name' | 'score'>>;
// TODO: fill the patch fields allowed by Pick and Partial
const patch: UserPatch = { name: 'Ada' };
console.log(`${patch.name}:${patch.score ?? 0}`);
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:5\n",
        }],
        TS_TYPE_REFS
    ),
    lesson!(
        "ts-discriminated-unions",
        "ts",
        "advanced",
        "Discriminated unions",
        "A shared literal field lets switch narrow each variant and makes missing cases visible during type checking.",
        r#"type Shape = { kind: 'rect'; width: number; height: number } | { kind: 'circle'; radius: number };
function measure(shape: Shape): number {
  switch (shape.kind) {
    case 'rect':
      return shape.width * shape.height;
    case 'circle':
      return shape.radius * 2;
  }
}

console.log('example:', measure({ kind: 'rect', width: 3, height: 4 }));"#,
        r#"type Shape = { kind: 'rect'; width: number; height: number } | { kind: 'circle'; radius: number };
function measure(shape: Shape): number {
  switch (shape.kind) {
    case 'rect':
      // TODO: use the fields that only exist on the rect variant
      return shape.width + shape.height;
    case 'circle':
      return shape.radius * 2;
  }
}

console.log(measure({ kind: 'rect', width: 3, height: 4 }));
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/narrowing.html#discriminated-unions",
            "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-4.html",
        ]
    ),
    lesson!(
        "ts-async-promise",
        "ts",
        "intermediate",
        "Async and Promise",
        "async functions return Promise values; await unwraps the fulfilled value before later code uses it.",
        r#"async function double(value: number): Promise<number> {
  return value * 2;
}

async function main(): Promise<void> {
  console.log('example:', await double(2));
}

main();"#,
        r#"async function double(value: number): Promise<number> {
  return value * 2;
}

async function main(): Promise<void> {
  // TODO: await the Promise before printing its number
  console.log(String(double(2)));
}

main();
"#,
        &[SyntaxCase {
            input: "",
            output: "4\n",
        }],
        &[
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await",
            "https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#functions-which-return-promises",
        ]
    ),
    lesson!(
        "ts-error-handling",
        "ts",
        "intermediate",
        "Error handling",
        "catch receives an unknown failure; narrow it before reading Error-specific fields or choosing a fallback.",
        r#"function parseCount(text: string): number {
  try {
    return Number.parseInt(text, 10);
  } catch (error: unknown) {
    return error instanceof Error ? 0 : -1;
  }
}

console.log('example:', parseCount('12'));"#,
        r#"function parseCount(text: string): number {
  try {
    const value = Number.parseInt(text, 10);
    if (Number.isNaN(value)) throw new Error('bad number');
    return value;
  } catch (error: unknown) {
    // TODO: narrow the caught value and recover with 12
    return error instanceof Error ? 0 : -1;
  }
}

console.log(parseCount('bad'));
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &[
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch",
            "https://www.typescriptlang.org/docs/handbook/2/narrowing.html",
        ]
    ),
    lesson!(
        "ts-modules",
        "ts",
        "intermediate",
        "Modules and exports",
        "import and export make file boundaries explicit; in Node type stripping, module syntax still follows Node's module rules.",
        r#"export function label(value: string): string {
  return value.toUpperCase();
}

console.log('example:', label('ok'));"#,
        r#"export function label(value: string): string {
  // TODO: export behavior that callers can trust
  return value;
}

console.log(label('ok'));
"#,
        &[SyntaxCase {
            input: "",
            output: "OK\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/modules.html",
            "https://nodejs.org/api/typescript.html#determining-module-system",
        ]
    ),
    lesson!(
        "ts-classes",
        "ts",
        "intermediate",
        "Classes and access modifiers",
        "Classes combine state with methods; TypeScript access modifiers describe the intended boundary for that state.",
        r#"class Counter {
  private value: number;

  constructor(start: number) {
    this.value = start;
  }

  increment(): number {
    this.value += 1;
    return this.value;
  }
}

console.log('example:', new Counter(1).increment());"#,
        r#"class Counter {
  private value: number;

  constructor(start: number) {
    this.value = start;
  }

  increment(): number {
    // TODO: update private state before returning it
    return this.value;
  }
}

console.log(new Counter(1).increment());
"#,
        &[SyntaxCase {
            input: "",
            output: "2\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/classes.html",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes",
            "https://nodejs.org/api/typescript.html#typescript-features",
        ]
    ),
    lesson!(
        "ts-readonly",
        "ts",
        "intermediate",
        "readonly",
        "readonly documents that callers may read a property or array but should not replace or mutate it through that type.",
        r#"type Config = { readonly name: string; readonly scores: readonly number[] };
const config: Config = { name: 'Ada', scores: [2, 3] };
console.log('example:', `${config.name}:${config.scores.reduce((sum, n) => sum + n, 0)}`);"#,
        r#"type Config = { readonly name: string; readonly scores: readonly number[] };
const config: Config = { name: 'Ada', scores: [2, 3] };
// TODO: read from readonly data without replacing it
console.log(`${config.name}:${config.scores.length}`);
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:5\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/2/objects.html#readonly-properties",
            "https://www.typescriptlang.org/docs/handbook/2/objects.html#the-readonlyarray-type",
        ]
    ),
    lesson!(
        "ts-satisfies-as-const",
        "ts",
        "advanced",
        "satisfies and as const",
        "as const preserves literal values, and satisfies checks a wider contract without widening the value's own type.",
        r#"const routes = {
  home: '/',
  user: '/users',
} as const satisfies Record<string, `/${string}`>;
type RouteName = keyof typeof routes;
const selected: RouteName = 'user';
console.log('example:', routes[selected]);"#,
        r#"const routes = {
  home: '/',
  user: '/users',
} as const satisfies Record<string, `/${string}`>;
type RouteName = keyof typeof routes;
// TODO: choose the route key that selects the user path
const selected: RouteName = 'home';
console.log(routes[selected]);
"#,
        &[SyntaxCase {
            input: "",
            output: "/users\n",
        }],
        &[
            "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-4-9.html",
            "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-0.html",
            "https://www.typescriptlang.org/docs/handbook/2/typeof-types.html",
        ]
    ),
    lesson!(
        "ts-iterables",
        "ts",
        "intermediate",
        "Iterables",
        "for...of consumes any iterable, so arrays, strings, sets, and many Node values can share loop code.",
        r#"const chars: Iterable<string> = ['o', 'k'];
let word = '';
for (const ch of chars) {
  word += ch;
}
console.log('example:', word);"#,
        r#"const chars: Iterable<string> = ['o', 'k'];
let word = '';
for (const ch of chars) {
  // TODO: collect every yielded character
  word = ch;
}
console.log(word);
"#,
        EMPTY_HELLO,
        &[
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...of",
            "https://www.typescriptlang.org/docs/handbook/iterators-and-generators.html",
        ]
    ),
    lesson!(
        "ts-array-methods",
        "ts",
        "intermediate",
        "map, filter, and reduce",
        "map transforms each item, filter keeps selected items, and reduce folds a sequence into one accumulated value.",
        r#"const nums = [1, 2, 3, 4];
const total = nums
  .filter((n) => n % 2 === 0)
  .map((n) => n * n)
  .reduce((sum, n) => sum + n, 0);
console.log('example:', total);"#,
        r#"const nums = [1, 2, 3, 4];
// TODO: square only the even numbers before summing
const total = nums.reduce((sum, n) => sum + n, 0);
console.log(total);
"#,
        &[SyntaxCase {
            input: "",
            output: "20\n",
        }],
        TS_ARRAY_REFS
    ),
];

const JAVA_LESSONS: &[SyntaxLesson] = &[
    lesson!(
        "java-output",
        "java",
        "basic",
        "Stdout",
        "System.out.print writes text as-is, while System.out.println appends a line break that judges usually compare exactly.",
        r#"class Solution {
    public static void main(String[] args) {
        int score = 7;
        System.out.println("example:" + ("score=" + score));
    }
}
"#,
        r#"class Solution {
    public static void main(String[] args) {
        int score = 7;
        // TODO: format the score for stdout with one trailing newline.
        System.out.println("TODO");
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "score=7\n",
        }],
        JAVA_CORE_REFS
    ),
    lesson!(
        "java-variables-types",
        "java",
        "basic",
        "Variables and types",
        "Local variables have declared types; primitives hold values directly and references point at objects such as String.",
        r#"class Solution {
    public static void main(String[] args) {
        String name = "Ada";
        int score = 7;
        boolean passed = score >= 5;
        System.out.println("example:" + (name + ":" + score + ":" + passed));
    }
}
"#,
        r#"class Solution {
    public static void main(String[] args) {
        String name = "Ada";
        int score = 0;
        boolean passed = false;
        // TODO: update the typed values before the report is formatted.
        System.out.println(name + ":" + score + ":" + passed);
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:7:true\n",
        }],
        JAVA_LANGUAGE_REFS
    ),
    lesson!(
        "java-numbers-operators",
        "java",
        "basic",
        "Numbers and operators",
        "Integer division, remainder, casts, and numeric promotion decide the value before it ever reaches stdout.",
        r#"class Solution {
    public static void main(String[] args) {
        int total = 17;
        int size = 5;
        System.out.println("example:" + ((total / size) + ":" + (total % size)));
    }
}
"#,
        r#"class Solution {
    public static void main(String[] args) {
        int total = 17;
        int size = 5;
        // TODO: use integer division and remainder together.
        System.out.println((total / 2) + ":" + (total - size));
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "3:2\n",
        }],
        JAVA_LANGUAGE_REFS
    ),
    lesson!(
        "java-strings",
        "java",
        "basic",
        "Strings",
        "String is immutable; methods such as trim, substring, charAt, and equals return or compare values without changing the original.",
        r#"class Solution {
    public static void main(String[] args) {
        String raw = "  ok!  ";
        String cleaned = raw.trim().substring(0, 2);
        System.out.println("example:" + (cleaned));
    }
}
"#,
        r#"class Solution {
    public static void main(String[] args) {
        String raw = "  ok!  ";
        // TODO: trim the text and keep only ok.
        String cleaned = raw.substring(0, 2);
        System.out.println(cleaned);
    }
}
"#,
        EMPTY_HELLO,
        JAVA_LANGUAGE_REFS
    ),
    lesson!(
        "java-control-flow",
        "java",
        "basic",
        "Control flow",
        "if selects a branch, loops repeat work, and each block must move the same typed values toward the final answer.",
        r#"class Solution {
    public static void main(String[] args) {
        int total = 0;
        for (int n = 1; n <= 3; n++) {
            if (n % 2 == 1) {
                total += n;
            }
        }
        System.out.println("example:" + (total));
    }
}
"#,
        r#"class Solution {
    public static void main(String[] args) {
        int total = 0;
        for (int n = 1; n <= 3; n++) {
            // TODO: add only odd numbers.
            total += n;
        }
        System.out.println(total);
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "4\n",
        }],
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/tutorial/java/nutsandbolts/flow.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-14.html",
        ]
    ),
    lesson!(
        "java-methods",
        "java",
        "basic",
        "Methods",
        "A method signature declares parameter types and a return type; return sends the computed value back to the caller.",
        r#"class Solution {
    static int area(int width, int height) {
        return width * height;
    }

    public static void main(String[] args) {
        System.out.println("example:" + (area(3, 4)));
    }
}
"#,
        r#"class Solution {
    static int area(int width, int height) {
        // TODO: return rectangle area, not perimeter.
        return width + height;
    }

    public static void main(String[] args) {
        System.out.println(area(3, 4));
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        JAVA_LANGUAGE_REFS
    ),
    lesson!(
        "java-input",
        "java",
        "intermediate",
        "Input parsing",
        "Coding-test Java usually reads System.in once, splits whitespace, then converts tokens before doing numeric work.",
        r#"import java.io.IOException;

class Solution {
    public static void main(String[] args) throws IOException {
        String input = new String(System.in.readAllBytes());
        int sum = 0;
        for (String token : input.trim().split("\\s+")) {
            if (!token.isEmpty()) {
                sum += Integer.parseInt(token);
            }
        }
        System.out.println("example:" + (sum));
    }
}
"#,
        r#"import java.io.IOException;

class Solution {
    public static void main(String[] args) throws IOException {
        String input = new String(System.in.readAllBytes());
        int sum = 0;
        // TODO: split input into tokens and add every parsed integer.
        if (input.isEmpty()) {
            sum = 0;
        }
        System.out.println(sum);
    }
}
"#,
        SUM_CASE,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/System.html#in",
            "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/Scanner.html",
        ]
    ),
    lesson!(
        "java-arrays-collections",
        "java",
        "intermediate",
        "Arrays and collections",
        "Arrays keep a fixed length, while List, Map, and Set cover growable order, keyed lookup, and uniqueness.",
        r#"import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

class Solution {
    public static void main(String[] args) {
        int[] nums = {2, 3};
        List<Integer> list = new ArrayList<>();
        Map<String, Integer> totals = new HashMap<>();
        Set<Integer> seen = new HashSet<>();
        for (int n : nums) {
            list.add(n);
            totals.merge("sum", n, Integer::sum);
            seen.add(n);
        }
        System.out.println("example:" + (totals.get("sum") + ":" + list.size() + ":" + seen.size()));
    }
}
"#,
        r#"import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

class Solution {
    public static void main(String[] args) {
        int[] nums = {2, 3};
        List<Integer> list = new ArrayList<>();
        Map<String, Integer> totals = new HashMap<>();
        Set<Integer> seen = new HashSet<>();
        for (int n : nums) {
            list.add(n);
            // TODO: update both the Map total and Set of seen values.
        }
        System.out.println(nums.length + ":" + list.size() + ":" + seen.size());
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "5:2:2\n",
        }],
        JAVA_COLLECTION_REFS
    ),
    lesson!(
        "java-classes-objects",
        "java",
        "basic",
        "Classes and objects",
        "A class defines fields and behavior; new creates an object whose instance methods read or change that state.",
        r#"class Counter {
    int value = 3;

    int add(int delta) {
        value += delta;
        return value;
    }
}

class Solution {
    public static void main(String[] args) {
        Counter counter = new Counter();
        System.out.println("example:" + (counter.add(2)));
    }
}
"#,
        r#"class Counter {
    int value = 3;

    int add(int delta) {
        // TODO: change this object's state by delta before returning it.
        return value;
    }
}

class Solution {
    public static void main(String[] args) {
        Counter counter = new Counter();
        System.out.println(counter.add(2));
    }
}
"#,
        SUM_CASE,
        JAVA_CLASS_REFS
    ),
    lesson!(
        "java-constructors",
        "java",
        "basic",
        "Constructors",
        "A constructor initializes each new object before methods run; overloaded constructors provide different entry points.",
        r#"class Rectangle {
    private final int width;
    private final int height;

    Rectangle(int width, int height) {
        this.width = width;
        this.height = height;
    }

    int area() {
        return width * height;
    }
}

class Solution {
    public static void main(String[] args) {
        System.out.println("example:" + (new Rectangle(3, 4).area()));
    }
}
"#,
        r#"class Rectangle {
    private final int width;
    private final int height;

    Rectangle(int width, int height) {
        this.width = width;
        // TODO: store the height parameter in the field.
        this.height = 0;
    }

    int area() {
        return width * height;
    }
}

class Solution {
    public static void main(String[] args) {
        System.out.println(new Rectangle(3, 4).area());
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/tutorial/java/javaOO/constructors.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
        ]
    ),
    lesson!(
        "java-encapsulation",
        "java",
        "basic",
        "Encapsulation",
        "private fields protect representation; public methods expose the operations callers are allowed to perform.",
        r#"class Score {
    private int points;

    void add(int delta) {
        if (delta > 0) {
            points += delta;
        }
    }

    int points() {
        return points;
    }
}

class Solution {
    public static void main(String[] args) {
        Score score = new Score();
        score.add(5);
        System.out.println("example:" + (score.points()));
    }
}
"#,
        r#"class Score {
    private int points;

    void add(int delta) {
        if (delta > 0) {
            // TODO: update the private field through this method.
        }
    }

    int points() {
        return points;
    }
}

class Solution {
    public static void main(String[] args) {
        Score score = new Score();
        score.add(5);
        System.out.println(score.points());
    }
}
"#,
        SUM_CASE,
        JAVA_CLASS_REFS
    ),
    lesson!(
        "java-static-members",
        "java",
        "basic",
        "Static members",
        "static fields and methods belong to the class, not one object, so they are shared through the class name.",
        r#"class Scale {
    static final int FACTOR = 3;

    static int apply(int value) {
        return value * FACTOR;
    }
}

class Solution {
    public static void main(String[] args) {
        System.out.println("example:" + (Scale.apply(2)));
    }
}
"#,
        r#"class Scale {
    static final int FACTOR = 3;

    static int apply(int value) {
        // TODO: use the shared FACTOR constant.
        return value;
    }
}

class Solution {
    public static void main(String[] args) {
        System.out.println(Scale.apply(2));
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "6\n",
        }],
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/tutorial/java/javaOO/classvars.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
        ]
    ),
    lesson!(
        "java-enum-switch",
        "java",
        "intermediate",
        "Enum and switch",
        "enum names a fixed set of constants, and switch expressions turn those constants into explicit result branches.",
        r#"enum Status {
    TODO, DONE
}

class Solution {
    static String label(Status status) {
        return switch (status) {
            case TODO -> "work";
            case DONE -> "ok";
        };
    }

    public static void main(String[] args) {
        System.out.println("example:" + (label(Status.DONE)));
    }
}
"#,
        r#"enum Status {
    TODO, DONE
}

class Solution {
    static String label(Status status) {
        return switch (status) {
            case TODO -> "work";
            // TODO: return ok for the DONE branch.
            case DONE -> "done";
        };
    }

    public static void main(String[] args) {
        System.out.println(label(Status.DONE));
    }
}
"#,
        EMPTY_HELLO,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-14.html",
        ]
    ),
    lesson!(
        "java-exceptions",
        "java",
        "intermediate",
        "Exceptions",
        "try/catch handles recoverable failures; checked exceptions must be caught or declared in a method signature.",
        r#"class Solution {
    static int parseOrDefault(String text) {
        try {
            return Integer.parseInt(text);
        } catch (NumberFormatException error) {
            return 12;
        }
    }

    public static void main(String[] args) {
        System.out.println("example:" + (parseOrDefault("bad")));
    }
}
"#,
        r#"class Solution {
    static int parseOrDefault(String text) {
        try {
            return Integer.parseInt(text);
        } catch (NumberFormatException error) {
            // TODO: recover in the NumberFormatException branch.
            return 0;
        }
    }

    public static void main(String[] args) {
        System.out.println(parseOrDefault("bad"));
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        JAVA_EXCEPTION_REFS
    ),
    lesson!(
        "java-generics",
        "java",
        "intermediate",
        "Generics",
        "Generics let one class or method preserve the caller's element type instead of falling back to Object casts.",
        r#"import java.util.List;

class Solution {
    static <T> T last(List<T> items) {
        return items.get(items.size() - 1);
    }

    public static void main(String[] args) {
        System.out.println("example:" + (last(List.of("skip", "ok"))));
    }
}
"#,
        r#"import java.util.List;

class Solution {
    static <T> T last(List<T> items) {
        // TODO: return the last element while preserving T.
        return items.get(0);
    }

    public static void main(String[] args) {
        System.out.println(last(List.of("skip", "ok")));
    }
}
"#,
        EMPTY_HELLO,
        &[
            "https://dev.java/learn/generics/",
            "https://docs.oracle.com/javase/tutorial/java/generics/index.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
        ]
    ),
    lesson!(
        "java-interfaces",
        "java",
        "intermediate",
        "Interfaces",
        "An interface names behavior a class promises to implement; callers can depend on that contract instead of the concrete class.",
        r#"interface Named {
    String name();
}

class User implements Named {
    public String name() {
        return "ok";
    }
}

class Solution {
    static String describe(Named named) {
        return named.name();
    }

    public static void main(String[] args) {
        System.out.println("example:" + (describe(new User())));
    }
}
"#,
        r#"interface Named {
    String name();
}

class User implements Named {
    public String name() {
        // TODO: satisfy the interface through the implementing method.
        return "";
    }
}

class Solution {
    static String describe(Named named) {
        return named.name();
    }

    public static void main(String[] args) {
        System.out.println(describe(new User()));
    }
}
"#,
        EMPTY_HELLO,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/tutorial/java/IandI/createinterface.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-9.html",
        ]
    ),
    lesson!(
        "java-inheritance-composition",
        "java",
        "intermediate",
        "Inheritance and composition",
        "Inheritance reuses an is-a relationship; composition keeps a helper object as a field when behavior is only a has-a dependency.",
        r#"class Bonus {
    int apply(int base) {
        return base + 2;
    }
}

class User {
    int baseScore() {
        return 3;
    }
}

class PremiumUser extends User {
    private final Bonus bonus = new Bonus();

    int score() {
        return bonus.apply(baseScore());
    }
}

class Solution {
    public static void main(String[] args) {
        System.out.println("example:" + (new PremiumUser().score()));
    }
}
"#,
        r#"class Bonus {
    int apply(int base) {
        return base + 2;
    }
}

class User {
    int baseScore() {
        return 3;
    }
}

class PremiumUser extends User {
    private final Bonus bonus = new Bonus();

    int score() {
        // TODO: compose Bonus with the inherited baseScore.
        return baseScore();
    }
}

class Solution {
    public static void main(String[] args) {
        System.out.println(new PremiumUser().score());
    }
}
"#,
        SUM_CASE,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/tutorial/java/IandI/subclasses.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
        ]
    ),
    lesson!(
        "java-records",
        "java",
        "advanced",
        "Records",
        "A record declares an immutable data carrier and gives you a constructor, accessors, equals, hashCode, and toString.",
        r#"record Point(int x, int y) {
    int sum() {
        return x + y;
    }
}

class Solution {
    public static void main(String[] args) {
        Point point = new Point(2, 3);
        System.out.println("example:" + (point.sum()));
    }
}
"#,
        r#"record Point(int x, int y) {
    int sum() {
        // TODO: use both generated accessors.
        return x();
    }
}

class Solution {
    public static void main(String[] args) {
        Point point = new Point(2, 3);
        System.out.println(point.sum());
    }
}
"#,
        SUM_CASE,
        &[
            "https://dev.java/learn/records/",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
        ]
    ),
    lesson!(
        "java-optional",
        "java",
        "intermediate",
        "Optional",
        "Optional<T> makes a maybe-present value explicit and asks the caller to map, filter, or provide a fallback.",
        r#"import java.util.Optional;

class Solution {
    public static void main(String[] args) {
        Optional<String> value = Optional.of("ok");
        String label = value.filter(text -> text.length() == 2).orElse("missing");
        System.out.println("example:" + (label));
    }
}
"#,
        r#"import java.util.Optional;

class Solution {
    public static void main(String[] args) {
        Optional<String> value = Optional.empty();
        // TODO: keep ok present and filter it before the fallback.
        String label = value.filter(text -> text.length() == 2).orElse("missing");
        System.out.println(label);
    }
}
"#,
        EMPTY_HELLO,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/Optional.html",
        ]
    ),
    lesson!(
        "java-streams-lambdas",
        "java",
        "advanced",
        "Streams and lambdas",
        "A lambda supplies behavior to a pipeline, and a stream processes elements only when a terminal operation consumes it.",
        r#"import java.util.List;

class Solution {
    public static void main(String[] args) {
        int total = List.of(1, 2, 3, 4).stream()
            .filter(n -> n % 2 == 0)
            .mapToInt(n -> n * n)
            .sum();
        System.out.println("example:" + (total));
    }
}
"#,
        r#"import java.util.List;

class Solution {
    public static void main(String[] args) {
        // TODO: square only the even numbers before summing.
        int total = List.of(1, 2, 3, 4).stream()
            .mapToInt(n -> n)
            .sum();
        System.out.println(total);
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "20\n",
        }],
        JAVA_STREAM_REFS
    ),
    lesson!(
        "java-comparators-sorting",
        "java",
        "intermediate",
        "Comparators and sorting",
        "Comparator objects define ordering for sorted collections and list sorting without changing the stored type.",
        r#"import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

record User(String name, int score) {}

class Solution {
    public static void main(String[] args) {
        List<User> users = new ArrayList<>(List.of(
            new User("Ada", 3),
            new User("Lin", 5),
            new User("Bo", 4)
        ));
        users.sort(Comparator.comparingInt(User::score).reversed());
        User best = users.get(0);
        System.out.println("example:" + (best.name() + ":" + best.score()));
    }
}
"#,
        r#"import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

record User(String name, int score) {}

class Solution {
    public static void main(String[] args) {
        List<User> users = new ArrayList<>(List.of(
            new User("Ada", 3),
            new User("Lin", 5),
            new User("Bo", 4)
        ));
        // TODO: sort by score descending, not by name.
        users.sort(Comparator.comparing(User::name));
        User best = users.get(0);
        System.out.println(best.name() + ":" + best.score());
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "Lin:5\n",
        }],
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/Comparator.html",
            "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/Collections.html",
        ]
    ),
    lesson!(
        "java-try-with-resources",
        "java",
        "intermediate",
        "Try-with-resources",
        "try-with-resources closes AutoCloseable values automatically after the block, even when reading or parsing fails.",
        r#"import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

class Solution {
    public static void main(String[] args) throws IOException {
        try (ByteArrayInputStream in = new ByteArrayInputStream("ok".getBytes(StandardCharsets.UTF_8))) {
            System.out.println("example:" + (new String(in.readAllBytes(), StandardCharsets.UTF_8)));
        }
    }
}
"#,
        r#"import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

class Solution {
    public static void main(String[] args) throws IOException {
        try (ByteArrayInputStream in = new ByteArrayInputStream("todo".getBytes(StandardCharsets.UTF_8))) {
            // TODO: read ok from the managed resource.
            System.out.println(new String(in.readAllBytes(), StandardCharsets.UTF_8));
        }
    }
}
"#,
        EMPTY_HELLO,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/tutorial/essential/exceptions/tryResourceClose.html",
            "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/AutoCloseable.html",
        ]
    ),
    lesson!(
        "java-packages-imports",
        "java",
        "basic",
        "Packages and imports",
        "A package names a namespace across files; in this single-file judge, imports are the practical way to use packaged JDK classes.",
        r#"import java.util.ArrayList;
import java.util.List;

class Solution {
    public static void main(String[] args) {
        List<String> words = new ArrayList<>();
        words.add("o");
        words.add("k");
        System.out.println("example:" + (String.join("", words)));
    }
}
"#,
        r#"import java.util.ArrayList;
import java.util.List;

class Solution {
    public static void main(String[] args) {
        List<String> words = new ArrayList<>();
        words.add("o");
        // TODO: add the second letter using the imported List implementation.
        System.out.println(String.join("", words));
    }
}
"#,
        EMPTY_HELLO,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/tutorial/java/package/index.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-7.html",
        ]
    ),
    lesson!(
        "java-annotations",
        "java",
        "advanced",
        "Annotations",
        "Annotations attach metadata to declarations; tools and frameworks can read them without changing normal method execution.",
        r#"@interface Audit {
    String value();
}

class Solution {
    @Audit("stdout")
    static String label() {
        return "ok";
    }

    public static void main(String[] args) {
        System.out.println("example:" + (label()));
    }
}
"#,
        r#"@interface Audit {
    String value();
}

class Solution {
    @Audit("stdout")
    static String label() {
        // TODO: keep the annotated method behavior correct.
        return "";
    }

    public static void main(String[] args) {
        System.out.println(label());
    }
}
"#,
        EMPTY_HELLO,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/tutorial/java/annotations/index.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-9.html",
        ]
    ),
    lesson!(
        "java-sealed-classes",
        "java",
        "advanced",
        "Sealed classes",
        "sealed restricts which classes or records may implement a hierarchy, making closed domain alternatives visible to readers.",
        r#"sealed interface Shape permits Rect, Dot {
    int measure();
}

record Rect(int width, int height) implements Shape {
    public int measure() {
        return width * height;
    }
}

record Dot() implements Shape {
    public int measure() {
        return 0;
    }
}

class Solution {
    public static void main(String[] args) {
        Shape shape = new Rect(3, 4);
        System.out.println("example:" + (shape.measure()));
    }
}
"#,
        r#"sealed interface Shape permits Rect, Dot {
    int measure();
}

record Rect(int width, int height) implements Shape {
    public int measure() {
        // TODO: compute rectangle area inside the permitted record.
        return width + height;
    }
}

record Dot() implements Shape {
    public int measure() {
        return 0;
    }
}

class Solution {
    public static void main(String[] args) {
        Shape shape = new Rect(3, 4);
        System.out.println(shape.measure());
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-9.html",
        ]
    ),
    lesson!(
        "java-testing-assert",
        "java",
        "intermediate",
        "Testing and assert",
        "Small testable methods let assertions check behavior before main prints; AssertionError is the simplest failure signal.",
        r#"class Solution {
    static int addTwo(int value) {
        return value + 2;
    }

    static void check() {
        if (addTwo(3) != 5) {
            throw new AssertionError("addTwo should add 2");
        }
    }

    public static void main(String[] args) {
        check();
        System.out.println("example:" + (addTwo(3)));
    }
}
"#,
        r#"class Solution {
    static int addTwo(int value) {
        // TODO: make the method satisfy the assertion before stdout.
        return value;
    }

    static void check() {
        if (addTwo(3) != 3) {
            throw new AssertionError("current starter expectation");
        }
    }

    public static void main(String[] args) {
        check();
        System.out.println(addTwo(3));
    }
}
"#,
        SUM_CASE,
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/AssertionError.html",
            "https://docs.oracle.com/javase/tutorial/essential/exceptions/",
        ]
    ),
    lesson!(
        "java-equality-hashcode",
        "java",
        "advanced",
        "equals and hashCode",
        "Hash-based collections rely on equals and hashCode agreeing about which objects represent the same value.",
        r#"import java.util.HashSet;
import java.util.Objects;
import java.util.Set;

class Point {
    private final int x;
    private final int y;

    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public boolean equals(Object other) {
        if (!(other instanceof Point point)) {
            return false;
        }
        return x == point.x && y == point.y;
    }

    public int hashCode() {
        return Objects.hash(x, y);
    }
}

class Solution {
    public static void main(String[] args) {
        Set<Point> points = new HashSet<>();
        points.add(new Point(2, 3));
        points.add(new Point(2, 3));
        System.out.println("example:" + (points.size()));
    }
}
"#,
        r#"import java.util.HashSet;
import java.util.Objects;
import java.util.Set;

class Point {
    private final int x;
    private final int y;

    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public boolean equals(Object other) {
        // TODO: compare Point values by fields, not object identity.
        return this == other;
    }

    public int hashCode() {
        return Objects.hash(x, y);
    }
}

class Solution {
    public static void main(String[] args) {
        Set<Point> points = new HashSet<>();
        points.add(new Point(2, 3));
        points.add(new Point(2, 3));
        System.out.println(points.size());
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "1\n",
        }],
        JAVA_COLLECTION_REFS
    ),
    lesson!(
        "java-overloading-varargs",
        "java",
        "advanced",
        "Overloading and varargs",
        "Overloading chooses a method by argument types, while varargs gathers remaining arguments into an array parameter.",
        r#"class Solution {
    static int total(int first, int... rest) {
        int sum = first;
        for (int value : rest) {
            sum += value;
        }
        return sum;
    }

    static String label(String name, int score) {
        return name + ":" + score;
    }

    public static void main(String[] args) {
        System.out.println("example:" + (label("Ada", total(2, 3))));
    }
}
"#,
        r#"class Solution {
    static int total(int first, int... rest) {
        int sum = first;
        for (int value : rest) {
            // TODO: include every varargs value.
        }
        return sum;
    }

    static String label(String name, int score) {
        return name + ":" + score;
    }

    public static void main(String[] args) {
        System.out.println(label("Ada", total(2, 3)));
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada:5\n",
        }],
        &[
            "https://dev.java/learn/",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html",
            "https://docs.oracle.com/javase/specs/jls/se21/html/jls-15.html",
        ]
    ),
];

const RUST_LESSONS: &[SyntaxLesson] = &[
    lesson!(
        "rust-output",
        "rust",
        "basic",
        "Output",
        "println! formats values and writes exactly one line to stdout.",
        r#"fn main() {
    let score = 7;
    println!("example:score={score}");
}"#,
        r#"fn main() {
    let score = 7;
    // TODO: format the value above for stdout
    println!("TODO");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "score=7\n",
        }],
        &["https://doc.rust-lang.org/std/macro.println.html"]
    ),
    lesson!(
        "rust-variables",
        "rust",
        "basic",
        "Bindings and mutability",
        "let creates immutable bindings by default; mut makes rebinding through the same name explicit.",
        r#"fn main() {
    let label = "sum";
    let mut total = 1;
    total += 2;
    println!("example:{label}:{total}");
}"#,
        r#"fn main() {
    let label = "TODO";
    let mut total = 1;
    // TODO: change total with mutation before formatting
    total += 0;
    println!("{label}:{total}");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "sum:3\n",
        }],
        &["https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html"]
    ),
    lesson!(
        "rust-numbers-tuples",
        "rust",
        "basic",
        "Numbers and tuples",
        "Numeric types are explicit when inference is not enough; tuples group a fixed number of different values.",
        r#"fn main() {
    let pair: (i32, i32) = (2, 3);
    let sum = pair.0 + pair.1;
    println!("example:{sum}");
}"#,
        r#"fn main() {
    let pair: (i32, i32) = (2, 3);
    // TODO: use both tuple fields in the calculation
    let sum = pair.0;
    println!("{sum}");
}
"#,
        SUM_CASE,
        &["https://doc.rust-lang.org/book/ch03-02-data-types.html"]
    ),
    lesson!(
        "rust-strings",
        "rust",
        "basic",
        "Strings",
        "String owns growable UTF-8 text; &str is a borrowed string slice into existing UTF-8 text.",
        r#"fn main() {
    let mut name = String::from("rust");
    name.push_str("ace");
    let prefix: &str = &name[..4];
    println!("example:{prefix}:{}", name.len());
}"#,
        r#"fn main() {
    let mut name = String::from("rust");
    // TODO: extend the owned String before slicing and formatting
    name.push_str("");
    let prefix: &str = &name[..4];
    println!("{prefix}:{}", name.len());
}
"#,
        &[SyntaxCase {
            input: "",
            output: "rust:7\n",
        }],
        &["https://doc.rust-lang.org/book/ch04-03-slices.html"]
    ),
    lesson!(
        "rust-control-flow",
        "rust",
        "basic",
        "Control flow",
        "if can produce a value, and loop forms such as for let you turn ranges or collections into accumulated results.",
        r#"fn main() {
    let n = 3;
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    let mut total = 0;
    for value in 1..=n {
        total += value;
    }
    println!("example:{parity}:{total}");
}"#,
        r#"fn main() {
    let n = 3;
    let parity = if n % 2 == 0 { "even" } else { "TODO" };
    let mut total = 0;
    // TODO: include 1, 2, and 3 in the sum
    for value in 1..n {
        total += value;
    }
    println!("{parity}:{total}");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "odd:6\n",
        }],
        &["https://doc.rust-lang.org/book/ch03-05-control-flow.html"]
    ),
    lesson!(
        "rust-functions",
        "rust",
        "basic",
        "Functions",
        "Function signatures name parameter types and return types; the last expression can be the returned value.",
        r#"fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("example:{}", area(3, 4));
}"#,
        r#"fn area(width: u32, height: u32) -> u32 {
    // TODO: return rectangle area, not perimeter
    width + height
}

fn main() {
    println!("{}", area(3, 4));
}
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &["https://doc.rust-lang.org/book/ch03-03-how-functions-work.html"]
    ),
    lesson!(
        "rust-structs-impl",
        "rust",
        "basic",
        "Structs and impl",
        "A struct names related fields, and an impl block attaches methods and associated functions to that type.",
        r#"struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 3, height: 4 };
    println!("example:{}", rect.area());
}"#,
        r#"struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // TODO: calculate from both fields
        self.width + self.height
    }
}

fn main() {
    let rect = Rectangle { width: 3, height: 4 };
    println!("{}", rect.area());
}
"#,
        &[SyntaxCase {
            input: "",
            output: "12\n",
        }],
        &["https://doc.rust-lang.org/book/ch05-00-structs.html"]
    ),
    lesson!(
        "rust-enum-match",
        "rust",
        "basic",
        "Enums and match",
        "Enums model a closed set of variants, and match forces each variant to be handled deliberately.",
        r#"enum Command {
    Add(i32, i32),
    Quit,
}

fn run(command: Command) -> i32 {
    match command {
        Command::Add(a, b) => a + b,
        Command::Quit => 0,
    }
}

fn main() {
    println!("example:{}", run(Command::Add(2, 3)));
}"#,
        r#"enum Command {
    Add(i32, i32),
    Quit,
}

fn run(command: Command) -> i32 {
    match command {
        // TODO: return the sum carried by Add
        Command::Add(_a, _b) => 0,
        Command::Quit => 0,
    }
}

fn main() {
    println!("{}", run(Command::Add(2, 3)));
}
"#,
        SUM_CASE,
        &["https://doc.rust-lang.org/book/ch06-00-enums.html"]
    ),
    lesson!(
        "rust-option",
        "rust",
        "basic",
        "Option and if let",
        "Option<T> makes absence explicit, so code must handle Some(value) and None instead of assuming a value exists.",
        r#"fn first_char(text: &str) -> Option<char> {
    text.chars().next()
}

fn main() {
    if let Some(ch) = first_char("rust") {
        println!("example:{ch}");
    } else {
        println!("example:empty");
    }
}"#,
        r#"fn first_char(text: &str) -> Option<char> {
    text.chars().next()
}

fn main() {
    // TODO: choose input that makes Some('r') flow through if let
    if let Some(ch) = first_char("") {
        println!("{ch}");
    } else {
        println!("empty");
    }
}
"#,
        &[SyntaxCase {
            input: "",
            output: "r\n",
        }],
        &["https://doc.rust-lang.org/std/option/enum.Option.html"]
    ),
    lesson!(
        "rust-modules-use",
        "rust",
        "basic",
        "Modules and use",
        "mod creates a namespace, pub exposes selected items, and use brings a path into local scope without changing ownership.",
        r#"mod scoring {
    pub fn label(score: u32) -> &'static str {
        if score >= 80 { "pass" } else { "retry" }
    }
}

use scoring::label;

fn main() {
    println!("example:{}", label(91));
}"#,
        r#"mod scoring {
    pub fn label(score: u32) -> &'static str {
        if score >= 80 { "pass" } else { "retry" }
    }
}

use scoring::label;

fn main() {
    // TODO: pass a score that selects pass
    println!("{}", label(10));
}
"#,
        &[SyntaxCase {
            input: "",
            output: "pass\n",
        }],
        &[
            "https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html"
        ]
    ),
    lesson!(
        "rust-input",
        "rust",
        "intermediate",
        "Input parsing",
        "Coding-test Rust usually reads stdin as text, splits it, parses tokens once, and solves with typed values.",
        r#"use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sum: i32 = input.split_whitespace()
        .map(|token| token.parse::<i32>().unwrap())
        .sum();
    println!("example:{sum}");
}"#,
        r#"use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // TODO: parse all integers from stdin and print their sum
    let sum = 0;
    println!("{sum}");
}
"#,
        SUM_CASE,
        &["https://doc.rust-lang.org/std/io/trait.Read.html"]
    ),
    lesson!(
        "rust-vec-hashmap",
        "rust",
        "intermediate",
        "Vec and HashMap",
        "Vec<T> stores ordered values, while HashMap<K, V> stores lookups by key; entry is the usual counting API.",
        r#"use std::collections::HashMap;

fn main() {
    let nums = vec![1, 2, 3];
    let mut counts = HashMap::new();
    for word in ["red", "blue", "red"] {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("example:{} {}", nums.iter().sum::<i32>(), counts["red"]);
}"#,
        r#"use std::collections::HashMap;

fn main() {
    let nums = vec![1, 2, 3];
    let mut counts = HashMap::new();
    for word in ["red", "blue", "red"] {
        // TODO: count each word with entry(...).or_insert(...)
        counts.insert(word, 1);
    }
    println!("{} {}", nums.len(), counts["red"]);
}
"#,
        &[SyntaxCase {
            input: "",
            output: "6 2\n",
        }],
        &[
            "https://doc.rust-lang.org/std/vec/struct.Vec.html",
            "https://doc.rust-lang.org/std/collections/struct.HashMap.html",
        ]
    ),
    lesson!(
        "rust-borrowing-slices",
        "rust",
        "intermediate",
        "Borrowing and slices",
        "Borrowed slices let functions read part of owned data without taking ownership of the whole value.",
        r#"fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let line = String::from("rust rules");
    println!("example:{}", first_word(&line));
}
"#,
        r#"fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let line = String::from("rust rules");
    // TODO: borrow the String so first_word can read it
    println!("{}", first_word(""));
}
"#,
        &[SyntaxCase {
            input: "",
            output: "rust\n",
        }],
        &["https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html"]
    ),
    lesson!(
        "rust-result",
        "rust",
        "intermediate",
        "Result and ?",
        "Result<T, E> represents recoverable failure; the ? operator unwraps Ok or returns the Err to the caller.",
        r#"fn parse_count(text: &str) -> Result<i32, std::num::ParseIntError> {
    text.parse::<i32>()
}

fn main() -> Result<(), std::num::ParseIntError> {
    let count = parse_count("3")?;
    println!("example:{}", count + 2);
    Ok(())
}"#,
        r#"fn parse_count(text: &str) -> Result<i32, std::num::ParseIntError> {
    text.parse::<i32>()
}

fn main() -> Result<(), std::num::ParseIntError> {
    // TODO: parse 3 and use ? instead of unwrap
    let count = parse_count("0")?;
    println!("{}", count + 2);
    Ok(())
}
"#,
        SUM_CASE,
        &["https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html"]
    ),
    lesson!(
        "rust-ownership",
        "rust",
        "advanced",
        "Ownership and borrowing",
        "Each owned value has one owner; moving transfers ownership, while borrowing lets code inspect data without taking it.",
        r#"fn describe(name: String) -> (String, usize) {
    let len = name.len();
    (name, len)
}

fn main() {
    let name = String::from("rust");
    let (name, len) = describe(name);
    println!("example:{name}:{len}");
}"#,
        r#"fn describe(name: String) -> (String, usize) {
    let len = name.len();
    (name, len)
}

fn main() {
    let name = String::from("");
    // TODO: move the owned String into describe and use the returned owner
    let (name, len) = describe(name);
    println!("{name}:{len}");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "rust:4\n",
        }],
        &["https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html"]
    ),
    lesson!(
        "rust-iterators",
        "rust",
        "intermediate",
        "Iterators and closures",
        "Iterators are lazy until consumed; closures let map, filter, and fold express local transformations.",
        r#"fn main() {
    let nums = [1, 2, 3, 4];
    let total: i32 = nums.iter()
        .filter(|n| **n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("example:{total}");
}"#,
        r#"fn main() {
    let nums = [1, 2, 3, 4];
    // TODO: square only the even numbers before summing
    let total: i32 = nums.iter().map(|n| n).sum();
    println!("{total}");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "20\n",
        }],
        &["https://doc.rust-lang.org/book/ch13-02-iterators.html"]
    ),
    lesson!(
        "rust-generics",
        "rust",
        "intermediate",
        "Generics",
        "Generics let one function or type work with many concrete types while preserving compile-time type checking.",
        r#"fn last_copy<T: Copy>(items: &[T]) -> Option<T> {
    items.last().copied()
}

fn main() {
    println!("example:{}", last_copy(&[1, 2, 3]).unwrap());
}"#,
        r#"fn last_copy<T: Copy>(items: &[T]) -> Option<T> {
    // TODO: return the last copied item
    let _ = items;
    None
}

fn main() {
    println!("{}", last_copy(&[1, 2, 3]).unwrap_or(0));
}
"#,
        &[SyntaxCase {
            input: "",
            output: "3\n",
        }],
        &["https://doc.rust-lang.org/book/ch10-01-syntax.html"]
    ),
    lesson!(
        "rust-traits",
        "rust",
        "intermediate",
        "Traits and bounds",
        "Traits describe shared behavior, and bounds say which behavior a generic function is allowed to rely on.",
        r#"trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    tasks: usize,
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.tasks)
    }
}

fn print_summary<T: Summary>(item: &T) {
    println!("example:{}", item.summarize());
}

fn main() {
    let user = User { name: String::from("Ada"), tasks: 3 };
    print_summary(&user);
}"#,
        r#"trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    tasks: usize,
}

impl Summary for User {
    fn summarize(&self) -> String {
        // TODO: include both fields in the summary
        self.name.clone()
    }
}

fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    let user = User { name: String::from("Ada"), tasks: 3 };
    print_summary(&user);
}
"#,
        &[SyntaxCase {
            input: "",
            output: "Ada: 3\n",
        }],
        &["https://doc.rust-lang.org/book/ch10-02-traits.html"]
    ),
    lesson!(
        "rust-lifetimes",
        "rust",
        "intermediate",
        "Lifetimes",
        "Lifetime annotations describe relationships between borrowed values; they do not make any value live longer.",
        r#"fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}

fn main() {
    println!("example:{}", longer("borrow", "rs"));
}"#,
        r#"fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    // TODO: return the longer borrowed string
    let _ = left;
    right
}

fn main() {
    println!("{}", longer("borrow", "rs"));
}
"#,
        &[SyntaxCase {
            input: "",
            output: "borrow\n",
        }],
        &["https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"]
    ),
    lesson!(
        "rust-traits-lifetimes",
        "rust",
        "advanced",
        "Trait objects and dyn dispatch",
        "Trait objects such as &dyn Trait allow values of different concrete types to be used through shared behavior.",
        r#"trait Draw {
    fn draw(&self) -> &'static str;
}

struct Button;

impl Draw for Button {
    fn draw(&self) -> &'static str {
        "button"
    }
}

fn render(item: &dyn Draw) -> &'static str {
    item.draw()
}

fn main() {
    let button = Button;
    println!("example:{}", render(&button));
}"#,
        r#"trait Draw {
    fn draw(&self) -> &'static str;
}

struct Button;

impl Draw for Button {
    fn draw(&self) -> &'static str {
        // TODO: return the label used by render
        "TODO"
    }
}

fn render(item: &dyn Draw) -> &'static str {
    item.draw()
}

fn main() {
    let button = Button;
    println!("{}", render(&button));
}
"#,
        &[SyntaxCase {
            input: "",
            output: "button\n",
        }],
        &["https://doc.rust-lang.org/book/ch18-02-trait-objects.html"]
    ),
    lesson!(
        "rust-testing",
        "rust",
        "intermediate",
        "Tests and assertions",
        "Rust test functions use #[test] and assertion macros; normal code still needs small pure functions that tests can call.",
        r#"fn add_two(n: i32) -> i32 {
    n + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two() {
        assert_eq!(add_two(3), 5);
    }
}

fn main() {
    println!("example:{}", add_two(3));
}"#,
        r#"fn add_two(n: i32) -> i32 {
    // TODO: make the function satisfy the test expectation
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two() {
        assert_eq!(add_two(3), 5);
    }
}

fn main() {
    println!("{}", add_two(3));
}
"#,
        SUM_CASE,
        &["https://doc.rust-lang.org/book/ch11-00-testing.html"]
    ),
    lesson!(
        "rust-smart-pointers",
        "rust",
        "advanced",
        "Smart pointers",
        "Smart pointers such as Box<T> own data with pointer-like behavior and can place values on the heap.",
        r#"fn main() {
    let boxed = Box::new(String::from("heap"));
    println!("example:{}", boxed.len());
}"#,
        r#"fn main() {
    // TODO: put heap text inside Box<String> and read through it
    let boxed = Box::new(String::from(""));
    println!("{}", boxed.len());
}
"#,
        &[SyntaxCase {
            input: "",
            output: "4\n",
        }],
        &["https://doc.rust-lang.org/book/ch15-00-smart-pointers.html"]
    ),
    lesson!(
        "rust-interior-mutability",
        "rust",
        "advanced",
        "Interior mutability",
        "RefCell<T> checks borrow rules at runtime, allowing mutation through an immutable owner when the design requires it.",
        r#"use std::cell::RefCell;

fn main() {
    let log = RefCell::new(Vec::new());
    log.borrow_mut().push("event");
    println!("example:{}", log.borrow().len());
}"#,
        r#"use std::cell::RefCell;

fn main() {
    let log: RefCell<Vec<&str>> = RefCell::new(Vec::new());
    // TODO: borrow mutably and push one event
    println!("{}", log.borrow().len());
}
"#,
        &[SyntaxCase {
            input: "",
            output: "1\n",
        }],
        &["https://doc.rust-lang.org/book/ch15-05-interior-mutability.html"]
    ),
    lesson!(
        "rust-concurrency",
        "rust",
        "advanced",
        "Threads and join",
        "thread::spawn moves work to another OS thread, and join waits for that thread's result.",
        r#"use std::thread;

fn main() {
    let handle = thread::spawn(|| "worker");
    println!("example:{}", handle.join().unwrap());
}"#,
        r#"use std::thread;

fn main() {
    let handle = thread::spawn(|| "worker");
    // TODO: use the joined worker result instead of a placeholder
    let _ = handle;
    println!("main");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "worker\n",
        }],
        &["https://doc.rust-lang.org/book/ch16-01-threads.html"]
    ),
    lesson!(
        "rust-shared-state",
        "rust",
        "advanced",
        "Shared state with Arc and Mutex",
        "Arc<T> shares ownership across threads, and Mutex<T> protects mutation so only one thread edits at a time.",
        r#"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let count = Arc::new(Mutex::new(1));
    let worker_count = Arc::clone(&count);
    let handle = thread::spawn(move || {
        *worker_count.lock().unwrap() += 1;
    });
    handle.join().unwrap();
    println!("example:{}", *count.lock().unwrap());
}"#,
        r#"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let count = Arc::new(Mutex::new(1));
    let worker_count = Arc::clone(&count);
    let handle = thread::spawn(move || {
        // TODO: lock and increment the shared count
        let _ = worker_count;
    });
    handle.join().unwrap();
    println!("{}", *count.lock().unwrap());
}
"#,
        &[SyntaxCase {
            input: "",
            output: "2\n",
        }],
        &["https://doc.rust-lang.org/book/ch16-03-shared-state.html"]
    ),
    lesson!(
        "rust-async-await",
        "rust",
        "advanced",
        "Async and await",
        "async creates a Future that can pause at await points; executing futures needs a runtime or executor.",
        r#"async fn label() -> &'static str {
    "ready"
}

fn main() {
    let future = label();
    drop(future);
    println!("example:future-created");
}"#,
        r#"async fn label() -> &'static str {
    "ready"
}

fn main() {
    let future = label();
    drop(future);
    // TODO: this single-file exercise creates a Future but does not run an async runtime
    println!("pending");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "future-created\n",
        }],
        &["https://doc.rust-lang.org/book/ch17-00-async-await.html"]
    ),
    lesson!(
        "rust-macros",
        "rust",
        "advanced",
        "macro_rules!",
        "macro_rules! matches token patterns at compile time and expands them into Rust code before type checking.",
        r#"macro_rules! greet {
    ($name:expr) => {
        format!("hi {}", $name)
    };
}

fn main() {
    println!("example:{}", greet!("Rust"));
}"#,
        r#"macro_rules! greet {
    ($name:expr) => {
        // TODO: expand to hi <name>
        format!("TODO {}", $name)
    };
}

fn main() {
    println!("{}", greet!("Rust"));
}
"#,
        &[SyntaxCase {
            input: "",
            output: "hi Rust\n",
        }],
        &["https://doc.rust-lang.org/book/ch20-05-macros.html"]
    ),
    lesson!(
        "rust-unsafe",
        "rust",
        "advanced",
        "Unsafe Rust",
        "unsafe enables operations the compiler cannot fully verify, but the programmer must still uphold Rust's safety rules.",
        r#"fn main() {
    let value = 7;
    let pointer = &value as *const i32;
    let read = unsafe { *pointer };
    println!("example:{read}");
}"#,
        r#"fn main() {
    let value = 7;
    let pointer = &value as *const i32;
    // TODO: read the raw pointer inside an unsafe block
    let read = 0;
    let _ = pointer;
    println!("{read}");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "7\n",
        }],
        &["https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html"]
    ),
    lesson!(
        "rust-cargo-workspaces",
        "rust",
        "advanced",
        "Cargo packages and workspaces",
        "Cargo manages packages, dependencies, tests, and workspaces; workspace commands let related crates build together.",
        r#"const CHECK_ALL: &str = "cargo check --workspace";

fn main() {
    println!("example:{CHECK_ALL}");
}"#,
        r#"const CHECK_ALL: &str = "cargo check";

fn main() {
    // TODO: format the workspace-wide check command constant
    println!("{CHECK_ALL}");
}
"#,
        &[SyntaxCase {
            input: "",
            output: "cargo check --workspace\n",
        }],
        &["https://doc.rust-lang.org/cargo/reference/workspaces.html"]
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

# MP Programming Language

[中文版](./README_zh.md)

## Overview

MP is a simple, dynamically-typed programming language designed for ease of learning and use. It features clean syntax,
straightforward semantics, and includes IDE support via LSP.

## Features

- **Dynamic Typing**: No type declarations required
- **Built-in Types**: Numbers (Int/Float), Strings, Booleans, Arrays, Objects, Functions, Nil
- **Control Flow**: If/else conditionals, while loops, break/continue
- **Functions**: User-defined functions with return statements
- **Block Expressions**: Code blocks with automatic return values
- **Structs**: User-defined compound types
- **REPL**: Interactive programming environment
- **Formatter**: Built-in code formatter
- **LSP Support**: Full Language Server Protocol implementation for IDE integration

## Syntax

### Keywords

| Keyword    | Description                |
|------------|----------------------------|
| `if`       | Conditional statement      |
| `else`     | Alternative branch         |
| `while`    | Loop statement             |
| `let`      | Variable declaration       |
| `fn`       | Function definition        |
| `struct`   | Structure definition       |
| `break`    | Exit loop early            |
| `continue` | Skip to next iteration     |
| `return`   | Return value from function |

### Data Types

| Type     | Description               | Example                       |
|----------|---------------------------|-------------------------------|
| Number   | Integer or floating-point | `42`, `3.14`                  |
| String   | Text sequence             | `"hello"`                     |
| Boolean  | True or false             | `true`, `false`               |
| Array    | Ordered collection        | `[1, 2, 3]`                   |
| Object   | Key-value pairs           | `{"key": "value"}`            |
| Function | Callable function         | `fn add(a, b) { ... }`        |
| Struct   | User-defined type         | `struct Person { name, age }` |
| Nil      | Empty value               | `nil`                         |

### Operators

| Operator | Description                     |
|----------|---------------------------------|
| `+`      | Addition / String concatenation |
| `-`      | Subtraction                     |
| `*`      | Multiplication                  |
| `/`      | Division                        |
| `==`     | Equal                           |
| `!=`     | Not equal                       |
| `>`      | Greater than                    |
| `<`      | Less than                       |
| `>=`     | Greater than or equal           |
| `<=`     | Less than or equal              |

### Built-in Functions

| Function             | Description                       |
|----------------------|-----------------------------------|
| `print(expr)`        | Print value to console            |
| `input()`            | Read string from console          |
| `int(value)`         | Convert to integer                |
| `float(value)`       | Convert to float                  |
| `str(value)`         | Convert to string                 |
| `len(collection)`    | Get length of string/array/object |
| `type(expr)`         | Get type of expression            |
| `random([min, max])` | Generate random number            |
| `push(array, item)`  | Add item to array                 |
| `pop(array)`         | Remove last item from array       |
| `time()`             | Get current timestamp             |

## Usage

### Running MP Programs

```bash
# Run a file
mp filename.mp

# Start REPL
mp
```

### REPL Commands

| Command | Description             |
|---------|-------------------------|
| `exit`  | Exit the REPL           |
| `help`  | Show available commands |
| `clear` | Clear the environment   |

## Examples

### Hello World

```
print("Hello, World!");
```

### Variables

```
let name = "Alice";
let age = 25;
let score = 98.5;
```

### Functions

```
fn add(a, b) {
    return a + b;
}

let result = add(1, 2);
```

### Conditionals

```
if (age >= 18) {
    print("Adult");
} else {
    print("Minor");
}
```

### Loops

```
let i = 0;
while (i < 5) {
    print(i);
    i = i + 1;
}
```

### Break and Continue

```
let j = 0;
while (j < 10) {
    j = j + 1;
    if (j == 3) {
        continue;
    }
    if (j == 7) {
        break;
    }
    print(j);
}
```

### Arrays

```
let arr = [1, 2, 3, 4, 5];
print(len(arr));
push(arr, 6);
let last = pop(arr);
```

### Objects

```
let person = {
    "name": "Bob",
    "age": 30
};
print(person["name"]);
```

### Structs

```
struct Point {
    x,
    y
}

let p = Point(10, 20);
print(p.x);
```

### Type Checking

```
let num = 42;
print(type(num));  // int

let text = "hello";
print(type(text));  // string
```

### Type Conversion

```
let strNum = "123";
let num = int(strNum);

let floatNum = float("3.14");
let strVal = str(42);
```

### Random Numbers

```
let dice = random(1, 7);
let randomFloat = random(10.0);
```

### Comments

```
// Single-line comment

/*
Multi-line
comment
*/
```

## Project Structure

```
mp_lang/
├── src/
│   ├── lexer/          # Lexical analysis
│   ├── parser/         # Syntax parsing
│   ├── runtime/        # Interpreter & evaluator
│   ├── lsp/            # Language Server Protocol
│   ├── formatter.rs    # Code formatter
│   └── lib.rs          # Core library
├── [examples](./examples)  # Sample MP programs
├── tests/              # Test suites
└── vscode-extension/   # VS Code plugin
```

## Building

```bash
# Build the project
cargo build --release

# Run tests
cargo test
```

## IDE Support

MP includes full LSP (Language Server Protocol) implementation with:

- **Autocompletion**: Intelligent code suggestions
- **Hover**: Display type information on hover
- **Go to Definition**: Navigate to symbol definitions
- **Find References**: Locate all usages of a symbol
- **Diagnostics**: Real-time error reporting
- **Inlay Hints**: Show variable types inline
- **Workspace Symbols**: Search across files
- **Code Formatting**: Automatic code styling

Install the VS Code extension from `vscode-extension/` for the best development experience.

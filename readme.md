# Mp programming language

## Introduction

Mp is a simple programming language that is designed to be easy to learn and
use. It is a dynamically-typed language with a focus on simplicity and
readability.

## Syntax

### Keywords

| Keyword | Description           |
| ------- | --------------------- |
| `if`    | conditional statement |
| `else`  | conditional statement |
| `while` | loop statement        |
| `let`   | variable declaration  |
| `fn`    | function definition   |

### Data types

| Type     | Description                             | Implemented | Example                          |
| -------- | --------------------------------------- | ----------- | -------------------------------- |
| ---      | ---                                     | ---         | ---                              |
| Number   | integer or float                        | Yes         | `10`, `3.14`                     |
| String   | sequence of characters                  | Yes         | `"hello world"`                  |
| Boolean  | true or false                           | Yes         | `true`, `false`                  |
| Array    | ordered collection of values            | Yes         | `[1, 2, 3]`                      |
| Object   | unordered collection of key-value pairs | No          | `{ "name": "Alice", "age": 25 }` |
| Function | user-defined function                   | Yes         | `fn add(a, b) { return a + b; }` |
| Nil      | empty value                             | Yes         | `nil`                            |

### Operators

| Operator | Description              | Implemented |
| -------- | ------------------------ | ----------- |
| `+`      | addition                 | Yes         |
| `-`      | subtraction              | Yes         |
| `*`      | multiplication           | Yes         |
| `/`      | division                 | Yes         |
| `%`      | modulo                   | No          |
| `==`     | equal                    | Yes         |
| `!=`     | not equal                | Yes         |
| `>`      | greater than             | Yes         |
| `<`      | less than                | Yes         |
| `>=`     | greater than or equal to | Yes         |
| `<=`     | less than or equal to    | Yes         |

### Built-in functions

| Function      | Description                              | Implemented |
| ------------- | ---------------------------------------- | ----------- |
| `print(expr)` | print the value of `expr` to the console | Yes         |
| `input()`     | read a string from the console           | Yes         |
| `len(str)`    | return the length of `str`               | Yes         |
| `int(str)`    | convert `str` to an integer              | No          |
| `float(str)`  | convert `str` to a float                 | No          |
| `str(num)`    | convert `num` to a string                | No          |
| `type(expr)`  | return the type of `expr` as a string    | No          |

### Statements

In Mp, any statement has a value of `nil` and cannot be assigned to a variable.

| Statement                                                                                         | Description           |
| ------------------------------------------------------------------------------------------------- | --------------------- |
| `let a = 10;`                                                                                     | variable declaration  |
| `fn add(a, b) { return a + b; }`                                                                  | function definition   |
| `1+1;`                                                                                            | expression statement  |
| `if (a > 10) { print("a is greater than 10"); } else { print("a is less than or equal to 10"); }` | conditional statement |
| `while (i < 10) { print(i); i = i + 1; }`                                                         | loop statement        |

- single-line [expressions](#Expressions) are also statements.
- The last expression in a block expression is automatically wrapped in a
  `return` statement.

### Expressions

Any expression has a value (which can be `nil`).

| Expression                    | Description      |
| ----------------------------- | ---------------- |
| `expr1 + expr2`               | addition         |
| `expr1 - expr2`               | subtraction      |
| `expr1 * expr2`               | multiplication   |
| `expr1 / expr2`               | division         |
| `expr1 % expr2`               | modulo           |
| `expr1 == expr2`              | equal            |
| `expr1 != expr2`              | not equal        |
| `{expr1; expr2; ...; exprn;}` | block expression |

## Examples

[examples/example.mp](examples/example.mp)

### Comments

```
// This is a single-line comment

/*
This is a
multi-line comment
*/
```

### Variable declaration

```
let a = 10;
```

### Function definition

```
fn add(a, b) {
    return a + b;
}
```

### Block expression

```
{
    let a = 10;
    let b = 20;
    return a + b;
}
```

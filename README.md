# Vyn (WIP) — Core Language Specification


## Table of Contents

- [1. General](#1-general)
- [2. Syntax](#2-syntax)
  - [2.1 Whitespace](#21-whitespace)
  - [2.2 Blocks](#22-blocks)
  - [2.3 Comments](#23-comments)
- [3. Variables](#3-variables)
  - [3.1 Declaration](#31-declaration)
  - [3.2 Locality](#32-locality)
  - [3.3 Mutability](#33-mutability)
  - [3.4 Access](#34-access)
  - [3.5 Mutability and Access Together](#35-mutability-and-access-together)
  - [3.6 Type Annotations](#36-type-annotations)
- [4. Control Flow](#4-control-flow)
  - [4.1 If / Elif / Else](#41-if--elif--else)
  - [4.2 While](#42-while)
  - [4.3 For](#43-for)
  - [4.4 Filtered For](#44-filtered-for)
  - [4.5 Scope](#45-scope)
- [5. Functions](#5-functions)
  - [5.1 Declaration](#51-declaration)
  - [5.2 Parameters](#52-parameters)
  - [5.3 Function Calls](#53-function-calls)
- [6. Operators](#6-operators)
  - [6.1 Arithmetic](#61-arithmetic)
  - [6.2 Comparison](#62-comparison)
  - [6.3 Logical](#63-logical)
  - [6.4 Bitwise](#64-bitwise)
- [7. Types](#7-types)
  - [7.1 Primitive Types](#71-primitive-types)
  - [7.2 Optional](#72-optional)
  - [7.3 Collections](#73-collections)
  - [7.4 Arrays and Iteration](#74-arrays-and-iteration)
- [8. Example](#8-example)

## 1. General

- Compiled language
- Statically typed
- Explicit type annotations are required for variable declarations and function parameters
- Functions declare their return types
- Explicit `return` is required to return a value
- Locality, mutability, and access are separate properties

---

## 2. Syntax

### 2.1 Whitespace

Whitespace is not significant except for separating tokens.

```rust
x = y
x=y
x   =   y
```

### 2.2 Blocks

Blocks are enclosed in `{}`.

```rust
if enabled {
    // code
}
```

### 2.3 Comments

Single-line comments begin with `//`. Multi-line comments are enclosed in `/*` and `*/`.

```rust
// single-line comment
mut x: int = 5 // trailing comment

/*
multiple lines
of comments
*/
```

---

## 3. Variables

### 3.1 Declaration

```rust
[locality] [access] [mutability] name: type = value
```

- `local` and `readWrite` are the defaults and may be written explicitly.
- The declared type is mandatory, even when the initializer makes it obvious.
- Use `mut` for a reassignable binding or `const` for one that cannot be reassigned.

```rust
mut x: int = 5
local readWrite mut y: int = 10
global readOnly const MAX_COUNT: int = 10
local writeOnly mut output: int = 0
```

### 3.2 Locality

| Modifier | Meaning |
| --- | --- |
| `local` | Local binding (default) |
| `global` | Global binding |

```rust
mut x: int = 5                 // implicit local
local mut y: int = 10
```

`local` and `global` determine where a binding is declared; neither changes its access or mutability.

### 3.3 Mutability

| Modifier | Meaning |
| --- | --- |
| `mut` | Reassignment is permitted when access allows writing |
| `const` | Reassignment is prohibited after initialization |

```rust
mut x: int = 5
x = 10                       // allowed

const y: int = 5
// y = 10                    // error: const
```

### 3.4 Access

| Modifier | Reading | Writing after initialization |
| --- | --- | --- |
| `readOnly` | Allowed | Prohibited |
| `writeOnly` | Prohibited | Allowed if `mut` |
| `readWrite` | Allowed | Allowed if `mut` |

`readWrite` is the default access mode. Explicit `readWrite` has the same meaning as omitting it. Initialization is permitted for every access mode.

```rust
readOnly mut observed: int = 10
mut counter: int = observed   // allowed: reading observed
// observed = 20             // error: readOnly

writeOnly mut output: int = 0
output = 42                  // allowed
// counter = output          // error: writeOnly

readWrite mut total: int = 0
total = total + 1            // allowed
```

### 3.5 Mutability and Access Together

Mutability determines whether a binding can be reassigned. Access determines whether an operation through that binding is permitted. Both must permit a write.

| Declaration | Read | Reassign |
| --- | --- | --- |
| `readOnly const` | Yes | No |
| `writeOnly const` | No | No |
| `readWrite const` | Yes | No |
| `readOnly mut` | Yes | No |
| `writeOnly mut` | No | Yes |
| `readWrite mut` | Yes | Yes |

`writeOnly const` is legal but has no useful reads or post-initialization writes.

### 3.6 Type Annotations

All variable declarations must specify a type, even when the type could be inferred from the assigned value.

```rust
mut count: int = 5
const greeting: str = "hello"
mut values: Vector<int> = [1, 2, 3]
```

Declarations without type annotations are invalid:

```rust
// mut count = 5            // error: missing type
// const greeting = "hi"    // error: missing type
```

---

## 4. Control Flow

### 4.1 If / Elif / Else

```rust
if score > 10 {
    score = score - 1
} elif score == 10 {
    score = score + 1
} else {
    score = 0
}
```

### 4.2 While

```rust
mut i: int = 0
while i < 5 {
    i = i + 1
}
```

### 4.3 For

```rust
for item in values {
    print(item)
}
```

### 4.4 Filtered For

`where` restricts which elements enter the loop body.

```rust
for item in values where item > 5 {
    print(item)
}
```

### 4.5 Scope

Loop variables are block-scoped.

---

## 5. Functions

### 5.1 Declaration

```rust
fn name(param: type, ...) => return_type {
    return value
}
```

The return arrow is `=>`.

```rust
fn add(a: int, b: int) => int {
    return a + b
}
```

### 5.2 Parameters

Parameter types are mandatory. Parameters are not reassignable unless marked `mut`. Access is `readWrite` by default but may be specified explicitly.

```rust
fn increment(mut value: int) => int {
    value = value + 1
    return value
}

fn inspect(readOnly value: int) => int {
    return value
}

fn update(readWrite mut value: int) => int {
    value = value + 1
    return value
}
```

Writing to a mutable parameter changes the parameter's local binding; it does not imply that the caller's binding is changed.

### 5.3 Function Calls

Arguments are passed positionally.

```rust
mut result: int = add(1, 2)
result = increment(result)
```

---

## 6. Operators

### 6.1 Arithmetic

| Operator | Meaning |
| --- | --- |
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |

### 6.2 Comparison

| Operator | Meaning |
| --- | --- |
| `==` | Equal |
| `!=` | Not equal |
| `>` | Greater than |
| `>=` | Greater than or equal |
| `<` | Less than |
| `<=` | Less than or equal |

### 6.3 Logical

| Operator | Meaning |
| --- | --- |
| `&&` | AND |
| \| | OR |
| `!` | NOT |

```rust
if enabled && count > 0 {
    count = count + 1
}
```

### 6.4 Bitwise

| Operator | Meaning |
| --- | --- |
| `&` | AND |
| \| | OR |
| `^` | XOR |

---

## 7. Types

### 7.1 Primitive Types

```text
int
float
bool
str
```

### 7.2 Optional

`Optional<T>` represents either a value of type `T` or the absence of a value.

- `Some(value)` contains a value.
- `None` denotes no value.

```rust
const found: Optional<int> = Some(5)
const missing: Optional<int> = None
```

### 7.3 Collections

`Vector<T>` is a dynamically sized, ordered collection. `Map<K, V>` is a key-value collection.

```rust
mut numbers: Vector<int> = [1, 2, 3, 4, 5]
mut names: Map<str, int> = Map<str, int>()
```

### 7.4 Arrays and Iteration

Bracketed literals provide collection values, and `for ... in` iterates over collection elements.

```rust
const numbers: Vector<int> = [1, 2, 3]
for number in numbers {
    print(number)
}
```

---

## 8. Example

```rust
global readOnly const MAX_COUNT: int = 10

fn add(a: int, b: int) => int {
    return a + b
}

fn findGreaterThanFive(readOnly nums: Vector<int>) => Optional<int> {
    for x in nums where x > 5 {
        return Some(x)
    }
    return None
}

fn main() => int {
    // Both local and readWrite are implicit.
    mut numbers: Vector<int> = [1, 2, 3, 6, 8]

    // Both defaults can also be written explicitly.
    local readWrite mut result: int = add(1, 2)
    local readOnly const threshold: int = 5
    local writeOnly mut last_result: int = 0

    for x in numbers where x > threshold {
        print(x)
    }

    const value: Optional<int> = findGreaterThanFive(numbers)
    const fallback: Optional<int> = Some(MAX_COUNT)

    last_result = result
    // result = last_result   // error: writeOnly

    if result > 0 {
        result = result + 1
    }

    return result
}
```

# Vyn — Core Language Specification (Draft v0.1)

## 1. General

- Compiled language
- Statically typed
- Type inference supported when unambiguous
- Explicit `return` required in functions

---

## 2. Syntax

### 2.1 Whitespace

Whitespace is not significant (except for separating tokens).

Equivalent:

`a = b`  
`a=b`  
`a   =    b`

---

### 2.2 Blocks

Blocks are defined using `{}`:

```
if condition {
  // code
}
```

---

### 2.3 Comments

Vyn supports single-line and multi-line comments.

#### Single-line

```
// this is a comment
mut x = 5 // comment after code
```

#### Multi-line

```
/*
this is a multi-line comment
it can span multiple lines
*/
```

#### Notes

- Comments can appear on their own line or after code
- Comments do not affect program behavior

---

## 3. Variables

### Declaration

```
[locality] [mutability] name [: type] = value
```

### Locality

- `local` (optional)
- `global`

Default is local.

```
mut x = 5
local mut y = 10
global mut z = 20
```

---

### Mutability

- `mut` → reassignment allowed
- `const` → reassignment forbidden

```
mut x = 5
x = 10

const y = 5
y = 10 // error
```

---

### Type Annotation

```
mut x: int = 5
```

- Optional if inferable
- Required if ambiguous

---

### Type Inference

Inferable:

```
x = 5
y = "hello"
```

Ambiguous:

```
x = []
x: List<int> = []
```

---

### Mutability Model

| Concept           | Controlled by |
| ----------------- | ------------- |
| Reassignment      | mut / const   |
| Internal mutation | type          |

```
const arr = List<int>()
arr.push(1)
arr = [] // error
```

---

## 4. Control Flow

### If / Elif / Else

```
if condition {
}
elif condition {
}
else {
}
```

---

### While

```
while condition {
}
```

---

### For

```
for item in collection {
}
```

---

### Filtered For

```
for item in collection where condition {
}
```

```
for x in numbers where x > 5 {
  print(x)
}
```

---

### Scope

Loop variables are block-scoped.

---

## 5. Functions

### Declaration

```
fn name(param: type, ...) -> return_type {
  return value
}
```

---

### Parameters

- Immutable by default
- `mut` enables modification

```
fn add(a: int, b: int) -> int {
  return a + b
}

fn increment(mut x: int) -> int {
  x = x + 1
  return x
}
```

---

### Argument Rules

| Variable Type | param | mut param |
| ------------- | ----- | --------- |
| const         | yes   | no        |
| mut           | yes   | yes       |

---

### Calls

Positional:

```
add(1, 2)
```

Named:

```
add(a=1, b=2)
```

Rule:

```
add(1, b=2)   // error
add(a=1, 2)   // error
```

---

## 6. Operators

### Comparison

- `==   // is equal to `
- `!=   // is not equal to `
- `>    // is larger than`
- `>=   // is larger than or equal to`
- `<    // is smaller than`
- `<=   // is smaller than or equal to`

---

### Logical

- `&    // AND`
- `|    // OR`
- `||   // Bitwise XOR`

```
a & b
a | b
a || b
```

---

## 7. Types

### Primitive

```
int
float
bool
str
```

---

### Optional

Optional<T> represents a value that may or may not exist.

- Some(value)
- None

None is only valid within Optional<T>.

---

### Collections

```
List<T>
Map<K, V>
```

#### List

- Dynamic size
- Default collection

#### Map

- Key-value collection
- Dynamic size

---

## 8. Design Principles

- Explicit return
- Mutability is explicit
- Local scope is default
- Keep core small and consistent
- Avoid unnecessary complexity

---

## 9. Example

```
mut numbers: List<int> = [1, 2, 3, 6, 8]

fn add(a: int, b: int) -> int {
  return a + b
}

fn findGreaterThanFive(nums: List<int>) -> Optional<int> {
  for x in nums {
    if x > 5 {
      return Some(x)
    }
  }
  return None
}

for x in numbers where x > 5 {
  print(x)
}

result = add(a=1, b=2)

value = findGreaterThanFive(numbers)

if value is Some {
  print("found value")
} else {
  print("no value found")
}
```


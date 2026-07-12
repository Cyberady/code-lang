# Changelog

All notable changes to the Code programming language will be documented in this file.

This project follows a milestone-based development process.

---

# Version 1.0 Draft

## Added

### Core Language

* Variables
* Constants
* Numbers
* Strings
* Unicode support
* String interpolation
* Booleans
* Null
* Functions
* Automatic return
* Early return
* Arrays
* Nested arrays
* Objects
* Nested objects
* Conditionals
* Loops

---

## Variables

* Assignment creates variables.
* No `let`.
* No `var`.
* Variables are case-sensitive.

---

## Constants

* `const` keyword.
* Immutable after creation.

---

## Numbers

* Single numeric type.
* No exposed integer or floating-point types.

---

## Strings

* Single quotes.
* Double quotes.
* Triple quoted multiline strings.
* Unicode support.
* String interpolation using `{}`.

---

## Booleans

* `true`
* `false`

No truthy or falsy values.

---

## Null

Single empty value.

```text
null
```

No:

* undefined
* nil
* None

---

## Functions

* `func` keyword.
* Automatic return.
* Optional early `return`.

---

## Arrays

* Array literals
* Nested arrays
* Index access
* Mutation
* `length`
* `add()`
* `remove()`
* `contains()`

---

## Objects

* Object literals
* Nested objects
* Dot notation
* Property mutation

Bracket notation intentionally omitted.

---

## Conditionals

* `if`
* `else if`
* `else`

Comparison operators:

* `==`
* `!=`
* `>`
* `<`
* `>=`
* `<=`

Logical operators:

* `and`
* `or`
* `not`

---

## Loops

* `for`
* `while`
* `break`
* `continue`
* `range()`

No C-style loops.

No `do...while`.

---

## Guiding Principles

* Humans First
* One Obvious Way
* Less Syntax
* Readability Over Cleverness
* Production Ready
* Batteries Included

---

Future releases will continue expanding the standard library, tooling, and ecosystem while preserving these principles.

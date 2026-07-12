# Code Language Specification

**Version:** 1.0 Draft

---

# Introduction

Code is a programming language designed around one goal:

> **Build production software with the simplest possible syntax.**

The language removes unnecessary syntax while remaining powerful enough for real-world applications.

---

# Design Principles

- Humans first.
- One obvious way.
- Less syntax.
- Readability over cleverness.
- Production-ready.
- Batteries included.

---

# Variables

Variables are created using assignment.

```code
name = "Aditya"

age = 22

score = 100
```

Rules:

- No `let`
- No `var`
- First assignment creates the variable.
- Later assignments update the variable.
- Variables are case-sensitive.

Example:

```code
user = "Aditya"

User = "Rahul"

USER = "Priya"
```

These represent three different variables.

Valid names:

```code
username
user_name
player1
_player
```

Invalid names:

```text
1player
user-name
user name
```

---

# Constants

Constants use the `const` keyword.

```code
const PI = 3.14159

const VERSION = "1.0"
```

Rules

- Must be initialized.
- Cannot be reassigned.

Example

```code
const PI = 3.14159

PI = 10
```

Result

```text
Error

Cannot modify constant 'PI'
```

---

# Numbers

Code exposes a single numeric type.

```text
number
```

Examples

```code
count = 10

price = 99.99

pi = 3.14159
```

There are no exposed integer or floating-point types.

The runtime may optimize internally.

---

# Strings

Supports both double quotes and single quotes.

```code
name = "Aditya"

city = 'Mumbai'
```

Both forms are equivalent.

---

## Multiline Strings

Triple double quotes create multiline strings.

```code
message = """
Hello

Welcome to Code.
"""
```

---

## String Interpolation

Interpolation is the preferred way to build strings.

```code
name = "Aditya"

age = 22

print("Hello {name}")

print("{name} is {age} years old.")
```

String concatenation is supported but discouraged when interpolation is clearer.

---

## Unicode

Strings are Unicode by default.

```code
greeting = "नमस्ते"

emoji = "🔥"

country = "日本"
```

---

# Booleans

The language has exactly two boolean values.

```code
true

false
```

There are no truthy or falsy values.

Valid:

```code
if age >= 18 {

}

if loggedIn {

}
```

Invalid:

```code
if age {

}

if name {

}
```

Conditions must always evaluate to a boolean.

---

# Null

The language has one empty value.

```code
null
```

Example

```code
user = null

if user == null {

    print("User not found")

}
```

There is no:

- undefined
- nil
- None

---

# Functions

Functions are declared using the `func` keyword.

```code
func greet(name) {

    print("Hello {name}")

}
```

---

## Parameters

Functions may accept zero or more parameters.

```code
func hello() {

    print("Hello")

}

func add(a, b) {

    a + b

}
```

---

## Function Calls

```code
greet("Aditya")

result = add(10, 20)
```

---

## Automatic Return

The final evaluated expression is automatically returned.

```code
func square(x) {

    x * x

}
```

---

## Early Return

```code
func divide(a, b) {

    if b == 0 {

        return null

    }

    a / b

}
```

---

## Version 1 Limitations

Not supported:

- Default parameters
- Named parameters
- Generic functions
- Function overloading
- Type annotations

---

# Arrays

Arrays store ordered collections.

---

## Creating Arrays

```code
numbers = [1, 2, 3]

names = [

    "Aditya",

    "Rahul",

    "Priya"

]

mixed = [

    "John",

    22,

    true,

    null

]
```

Arrays may contain mixed value types.

---

## Nested Arrays

```code
matrix = [

    [1, 2],

    [3, 4]

]
```

---

## Access

```code
names[0]

names[1]
```

---

## Update

```code
names[0] = "Aryan"
```

---

## Length

```code
names.length
```

`length` is a property.

---

## Methods

```code
names.add("Rohit")

names.remove(1)

names.contains("Rahul")
```

Properties are nouns.

Methods are verbs.

---

## Empty Array

```code
users = []
```

---

# Objects

Objects represent related data.

---

## Creating Objects

```code
user = {

    name: "Aditya",

    age: 22,

    city: "Mumbai"

}
```

---

## Nested Objects

```code
user = {

    name: "Aditya",

    address: {

        city: "Mumbai",

        country: "India"

    }

}
```

---

## Empty Object

```code
user = {}
```

---

## Property Access

Only dot notation is supported.

```code
user.name

user.age

user.address.city
```

Bracket notation is intentionally omitted from Version 1.

---

## Updating Properties

```code
user.name = "Aryan"

user.age = 23
```

---

## Object Principles

- Objects are unordered key-value collections.
- Dot notation is the only property access syntax.
- Properties are nouns.
- Methods are verbs.

---

# Conditionals

---

## if

```code
if age >= 18 {

    print("Adult")

}
```

---

## else

```code
if age >= 18 {

    print("Adult")

} else {

    print("Minor")

}
```

---

## else if

```code
if marks >= 90 {

    print("A")

} else if marks >= 80 {

    print("B")

} else {

    print("C")

}
```

---

## Nested Conditions

```code
if loggedIn {

    if isAdmin {

        print("Welcome Admin")

    }

}
```

---

## Comparison Operators

Supported operators

```text
==
!=
>
<
>=
<=
```

---

## Logical Operators

Code uses readable keywords.

```text
and
or
not
```

Example

```code
if age >= 18 and verified {

    print("Access Granted")

}

if not loggedIn {

    login()

}
```

---

## Conditional Rules

- Parentheses are never required.
- Braces are always required.
- Conditions must evaluate to a boolean.

---

# Loops

Code provides only two looping constructs.

- for
- while

---

## For Loop

```code
for user in users {

    print(user)

}
```

---

## Nested Loops

```code
for row in matrix {

    for value in row {

        print(value)

    }

}
```

---

## Numeric Iteration

Numeric iteration uses the built-in `range()` function.

```code
for i in range(10) {

    print(i)

}
```

```code
for i in range(1, 10) {

    print(i)

}
```

```code
for i in range(5, 100, 5) {

    print(i)

}
```

---

## While Loop

```code
while connected {

    receive()

}
```

---

## break

```code
for user in users {

    if user.name == "Admin" {

        break

    }

}
```

---

## continue

```code
for user in users {

    if not user.active {

        continue

    }

    print(user)

}
```

---

## Loop Rules

Supported

- for
- while
- break
- continue

Not Supported

- do...while
- C-style for loops

---

# Core Philosophy Summary

- One obvious way.
- Less syntax.
- Readability over cleverness.
- Variables require no declaration keyword.
- Only one numeric type.
- Only one empty value (`null`).
- No truthy or falsy values.
- Dot notation only.
- Braces are always required.
- Parentheses are never required around conditions.
- Collections use `for item in collection`.
- Numeric iteration uses `range()`.

---

**End of Version 1.0 Draft**

# Code Roadmap

This roadmap outlines the long-term vision for the Code programming language.

The roadmap is divided into milestones.

Features may move between milestones as development progresses.

---

# Current Status

**Project Stage**

🟢 Language Design

The language syntax and philosophy are currently being designed before implementation begins.

---

# Version 0.1 — Language Foundation

## Core Language

* ✅ Variables
* ✅ Constants
* ✅ Numbers
* ✅ Strings
* ✅ Booleans
* ✅ Null
* ✅ Functions
* ✅ Arrays
* ✅ Objects
* ✅ Conditionals
* ✅ Loops

---

## Remaining Core Features

* ⬜ Comments
* ⬜ Imports
* ⬜ Modules
* ⬜ Error Handling
* ⬜ Enumerations
* ⬜ Pattern Matching (if approved)
* ⬜ Standard Library Design

---

# Version 0.2 — Standard Library

The first standard library.

## Collections

* Array utilities
* Object utilities
* String utilities

---

## Files

* Read files
* Write files
* Copy files
* Move files
* Delete files

---

## JSON

* Parse JSON
* Generate JSON

---

## Date & Time

* Current time
* Formatting
* Duration
* Time zones

---

## Random

* Random numbers
* Random strings
* UUID generation

---

# Version 0.3 — Modules

Project organization.

* Import system
* Module system
* Package structure
* Visibility rules

Example:

```code
import math

import http

import database
```

---

# Version 0.4 — Tooling

Developer experience.

## CLI

Official command line interface.

Example

```text
code run app.code

code build

code fmt

code test
```

---

## Formatter

Official formatter.

No third-party formatter required.

---

## Linter

Official linter.

Clear and educational error messages.

---

## Language Server

IDE support.

* Autocomplete
* Hover documentation
* Go to Definition
* Rename Symbol
* Diagnostics

---

# Version 0.5 — Testing

Built-in testing.

Example

```code
test "Addition works" {

    expect(add(2, 3)).toBe(5)

}
```

---

# Version 0.6 — Networking

Networking built into the standard library.

Examples

* HTTP Client
* HTTP Server
* WebSocket
* REST helpers

---

# Version 0.7 — Database

Official database support.

Examples

* SQLite
* PostgreSQL
* MySQL

Simple API.

No ORM required for basic usage.

---

# Version 0.8 — Package Manager

Official package manager.

Example

```text
code install auth

code update

code publish
```

---

# Version 0.9 — Production

Performance improvements.

Goals

* Faster compiler
* Faster runtime
* Better memory usage
* Better diagnostics

---

# Version 1.0

The first stable release.

Requirements

* Stable syntax
* Complete standard library
* Stable package manager
* Testing framework
* Formatter
* Documentation
* Cross-platform support

Version 1.0 means developers can confidently build production software.

---

# Version 2.0

Language evolution.

Potential improvements

* Better concurrency
* Performance optimizations
* Improved compiler
* Additional standard library modules

No breaking changes unless absolutely necessary.

---

# Long-Term Vision

Code should become more than a programming language.

It should become a complete ecosystem.

The ecosystem includes:

* Programming Language
* Compiler
* Runtime
* Package Manager
* Formatter
* Linter
* Language Server
* Testing Framework
* Documentation Generator
* Web Framework
* Database Toolkit
* AI SDK
* Cloud Deployment Tools

---

# Guiding Rule

New features are accepted only if they satisfy the manifesto.

Every addition must improve:

* Simplicity
* Readability
* Consistency
* Productivity

Otherwise, it does not belong in Code.

---

# Philosophy

We are not racing to Version 1.0.

We are building a language that developers can trust for years.

Quality is always more important than speed.

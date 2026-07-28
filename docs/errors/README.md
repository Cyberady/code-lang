# Code Language Error Guide

Welcome to the official error guide for the **Code** programming language.

Errors are a normal part of programming. They help identify problems in your source code and provide information on how to fix them.

Every documented error includes:

- A unique error code
- A description of the error
- Why the error occurs
- Incorrect examples
- Correct examples
- An explanation
- How to fix the error
- Related documentation

This guide should be used alongside the Syntax Guide whenever you encounter an error while writing Code programs.

---

# Error Categories

The Code language reports errors from three parts of the compiler.

## Interpreter Errors (E1000)

Errors that occur while executing a program.

These usually happen because the program performs an invalid operation at runtime.

Examples:

- Undefined variables
- Reassigning constants
- Invalid operations
- Runtime errors

---

## Lexer Errors (E2000)

Errors that occur while reading the source code.

These happen before parsing begins and usually indicate invalid characters or malformed literals.

Examples:

- Unexpected characters
- Unterminated strings
- Unterminated comments
- Invalid numeric literals

---

## Parser Errors (E3000)

Errors that occur while parsing tokens into syntax.

These indicate invalid program structure.

Examples:

- Unexpected tokens
- Unexpected end of file

---

# Error Code Reference

## Interpreter Errors

| Error Code | Description | Documentation |
|------------|-------------|---------------|
| E1001 | Undefined Variable | 📄 [undefined_variable.code](./undefined_variable.code) |
| E1002 | Cannot Assign Constant | 📄 [constant_assignment.code](./constant_assignment.code) |
| E1003 | Invalid Binary Operation | 📄 [invalid_operation.code](./invalid_operation.code) |
| E1004 | Runtime Error | 📄 [runtime_error.code](./runtime_error.code) |

---

## Lexer Errors

| Error Code | Description | Documentation |
|------------|-------------|---------------|
| E2001 | Unexpected Character | 📄 [unexpected_character.code](./unexpected_character.code) |
| E2002 | Unterminated String | 📄 [unterminated_string.code](./unterminated_string.code) |
| E2003 | Unterminated Comment | 📄 [unterminated_comment.code](./unterminated_comment.code) |
| E2004 | Invalid Numeric Literal | 📄 [invalid_number.code](./invalid_number.code) |

---

## Parser Errors

| Error Code | Description | Documentation |
|------------|-------------|---------------|
| E3001 | Unexpected Token | 📄 [unexpected_token.code](./unexpected_token.code) |
| E3002 | Unexpected End Of File | 📄 [unexpected_eof.code](./unexpected_eof.code) |

---

# How To Read An Error

A typical Code error looks like this:

```text
[E1001]: Undefined variable 'name'
```

The error consists of two parts:

- **Error Code** — A unique identifier for the error.
- **Description** — A human-readable explanation of the problem.

Use the error code to quickly locate the corresponding documentation page.

---

# Finding Solutions

When an error occurs:

1. Read the error message carefully.
2. Identify the error code.
3. Open the corresponding documentation page.
4. Review the explanation and examples.
5. Apply the suggested fix.
6. Run your program again.

---

# Documentation Structure

```text
docs/
└── errors/
    ├── README.md
    ├── undefined_variable.code
    ├── constant_assignment.code
    ├── invalid_operation.code
    ├── runtime_error.code
    ├── unexpected_character.code
    ├── unterminated_string.code
    ├── unterminated_comment.code
    ├── invalid_number.code
    ├── unexpected_token.code
    └── unexpected_eof.code
```

---

# Related Documentation

If you're new to the language, start with the Syntax Guide before reading this documentation.

📄 [Syntax Guide](../syntax/README.md)

---

Happy coding! 🚀

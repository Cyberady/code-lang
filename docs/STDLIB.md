# Code Standard Library

The Code Standard Library provides the essential tools required for modern software development.

Its purpose is to help developers build applications without depending on numerous third-party packages.

The standard library follows the same philosophy as the language itself:

- Human-friendly
- Consistent
- Production-ready
- Easy to learn
- Batteries included

---

# Design Principles

The standard library follows a few simple rules.

## Properties are nouns

Properties describe information.

```code
text.length

users.length

time.year
```

Properties never perform actions.

---

## Methods are verbs

Methods perform actions.

```code
text.upper()

users.add(user)

file.read(path)
```

Methods should clearly describe what they do.

---

## One Obvious Way

There should be one clear way to perform common tasks.

Avoid duplicate APIs with different names.

Good

```code
users.add(user)

text.replace("a", "b")
```

Avoid

```code
append()

push()

insertEnd()
```

---

## Simple Names

Names should be short without becoming unclear.

Good

```code
upper()

trim()

split()

clear()

copy()
```

Avoid unnecessary prefixes.

Bad

```code
toUpperCase()

trimWhitespace()

splitString()
```

---

# Core Modules

The following modules are part of every Code installation.

- String
- Array
- Object
- Math
- Time
- File
- JSON
- Console
- System

---

# String

Strings represent immutable Unicode text.

## Properties

```code
text.length
```

## Methods

```code
text.upper()

text.lower()

text.trim()

text.contains("hello")

text.startsWith("Hello")

text.endsWith("!")

text.replace("old", "new")

text.split(",")

text.repeat(3)
```

Example

```code
name = "aditya"

print(name.upper())
```

---

# Array

Arrays store ordered collections.

## Properties

```code
users.length

users.isEmpty
```

## Methods

```code
users.add(user)

users.insert(0, user)

users.remove(0)

users.clear()

users.contains(user)

users.first()

users.last()

users.reverse()

users.sort()

users.join(", ")

users.slice(1, 5)

users.clone()
```

Example

```code
numbers = [1,2,3]

numbers.add(4)
```

---

# Object

Objects store named values.

## Properties

```code
user.length
```

## Methods

```code
user.keys()

user.values()

user.entries()

user.has("email")

user.remove("age")

user.clear()

user.clone()
```

Example

```code
user = {

    name: "Aditya",

    age: 22

}

print(user.keys())
```

---

# Math

Provides mathematical constants and functions.

## Constants

```code
math.PI

math.E
```

## Methods

```code
math.abs()

math.sqrt()

math.pow()

math.round()

math.floor()

math.ceil()

math.max()

math.min()

math.random()
```

Example

```code
print(math.sqrt(25))
```

---

# Time

Provides date and time utilities.

## Properties

```code
time.year

time.month

time.day
```

## Methods

```code
time.now()

time.today()

time.sleep()

time.format()
```

---

# File

Provides access to the file system.

## Methods

```code
file.read(path)

file.write(path, text)

file.append(path, text)

file.exists(path)

file.delete(path)

file.copy(source, destination)

file.move(source, destination)

file.create(path)
```

---

# JSON

Provides JSON encoding and decoding.

## Methods

```code
json.encode(data)

json.decode(text)
```

---

# Console

Provides terminal interaction.

## Methods

```code
print()

input()

clear()
```

---

# System

Provides operating system information.

## Properties

```code
system.os

system.arch

system.home
```

## Methods

```code
system.exit()

system.env(name)
```

---

# Extended Modules

The following modules are planned for future versions.

## HTTP

```code
http.get(url)

http.post(url, body)

http.put(url, body)

http.delete(url)
```

---

## Database

```code
db.connect()

db.query()

db.execute()

db.close()
```

---

## Cryptography

```code
crypto.hash()

crypto.encrypt()

crypto.decrypt()

crypto.random()
```

---

## Compression

```code
zip.compress()

zip.extract()
```

---

## Logging

```code
log.info()

log.warn()

log.error()

log.debug()
```

---

## Testing

```code
test()

assert()

expect()
```

---

## XML

```code
xml.encode()

xml.decode()
```

---

## CSV

```code
csv.read()

csv.write()
```

---

## YAML

```code
yaml.encode()

yaml.decode()
```

---

## Image

```code
image.open()

image.resize()

image.save()
```

---

## Audio

```code
audio.open()

audio.play()

audio.save()
```

---

## Video

```code
video.open()

video.encode()
```

---

## AI

```code
ai.chat()

ai.embed()

ai.generate()

ai.image()
```

---

# Standard Library Philosophy

The standard library should solve the majority of everyday programming tasks.

Developers should spend their time building software instead of searching for basic libraries.

The library should remain:

- Small
- Consistent
- Easy to learn
- Production-ready
- Stable

Every new API should follow the same principles as the language itself:

- One obvious way.
- Less syntax.
- Readability over cleverness.
- Humans first.

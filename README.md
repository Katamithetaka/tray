# tray

A basic interpreter

## Current features:

- Parsing String and char literals
- Parsing a file and executing the code inside of it, line by line (not tested)
- Doing math operations, in the right order (with parenthesis and order of operation)
- Semi-strong type system: Numbers cannot interact with strings unless explicitely told so, but they are automatically converted between floating point types and integral types.

## To be implemented

- Keywords / Statements
- Variables
- Functions
- Dot operator (to call functions on types)
- Objects (C++/C# style)
- Error messages for parser and executioner without using panic!


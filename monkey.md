# Monkey Language

Source: two great books `Writing an Interpreter in Go` and `Writing a Compiler in Go`
by Thorsten Ball

Monkey programming language
``` 
let one = 1;
let two = 2;
let result = one + two;
puts(result)
```

Interpreter
``` 
Code (String)
--> Tokenizing with Lexer
--> Lexer(input string).NextToken() token.Token
Tokens 
--> Parsing with Parser
--> Parser(lexer).ParseProgram() *ast.RootNode
AST (ast.Node)
--> Evaluating with Tree-walking interpreter
--> func Eval(node ast.Node) object.Object
Result (Object)
```

```
package main

code := ""
lexer := Lexer.New(code)
parser := Parser.New(lexer)
programRootNode := parser.ParserProgram()
Eval(programRootNode)
```

## Module Structure

``` 
ast - Abstract Syntax Tree nodes output of the parser
compiler - converting AST to bytecodes (instructions in bytes)
eval - interpreter evaluation
lexer - lexing text to tokens
parser - parsing tokens to Abstract Syntax Tree
token - code tokens parsed from string
vm - virtual machine to execute the instructions in bytes from compiler
```

## Terms

- lexing
    - lexer
    - tokens
    - repl
- parsing
    - parser
    - abstract syntax tree (ast) node
        - program
        - statement
        - identifier
        - expression
            - prefix expr
            - infix expr
            - if expr
                - condition
                - consequence
                - alternative
            - integer literal
            - boolean literal
    - methods
        - top down
            - pratt parser (operator precedence)
        - bottom up
- interpreting
    - evaluation
    - environment
    - conditionals
    - objects
        - null
- compiling
    - compiler
    - bytecode
    - instructions
    - constants
    - opcodes
        - constant
        - pop
        - add
        - sub
        - mul
        - div
        - mod
        - true
        - false
        - equal
        - greater
        - and
        - or
        - minus
        - plus plus
        - minus minus
        - bang
        - jump not truthy
        - jump
        - null
        - get global
        - set global
        - array
        - hash
        - index
        - call
        - return value
        - return
        - get local
        - set local
        - get builtin
        - closure
        - get free
        - current closure
    - virtual machine
        - stack machine
        - stack pointer
    - objects
        - null
    - conditionals
        - ast flatten
        - jumps
        - null
    - bindings
    - closure
- symbol table
- built-in function


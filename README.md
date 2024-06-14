# Writing an interpreter and a compiler

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
  - tokens
  - lexer
  - repl
- Data structures
- Token
- AST
- Lexer
- Parser
- `Environment`
  - keeping track of which name is mapped to which value e.g. `let x=5;`
  - HashMap[string, Object]
  - get(name: string) -> Option[Object]
  - set(name: string, obj: Object) -> Result<()>
  - pass to Eval `eval(ast.Node, Environment) -> Object`
- infix operator
- prefix operator
- symbol table
- built-in function
- closure
- virtual machine: a computer built with software, mimics how a computer works


## References
Books
- Abelson, Harold and Sussman, Gerald Jay with Sussman, Julie. 1996. Structure and Interpretation of Computer Programs, Second Edition. MIT Press.
- Appel, Andrew W.. 2004. Modern Compiler Implementation in C. Cambridge University Press.
- Cooper, Keith D. and Torczon Linda. 2011. Engineering a Compiler, Second Edition. Morgan Kaufmann.
- Grune, Dick and Jacobs, Ceriel. 1990. Parsing Techniques. A Practical Guide.. Ellis Horwood Limited.
- Grune, Dick and van Reeuwijk, Kees and Bal Henri E. and Jacobs, Ceriel J.H. Jacobs and Langendoen, Koen. 2012. Modern Compiler Design, Second Edition. Springer
- Nisan, Noam and Schocken, Shimon. 2008. The Elements Of Computing Systems. MIT Press.

Papers
- Ayock, John. 2003. A Brief History of Just-In-Time. In ACM Computing Surveys, Vol. 35, No. 2, June 2003
- Ertl, M. Anton and Gregg, David. 2003. The Structure and Performance of Efficient Interpreters. In Journal Of Instruction-Level Parallelism 5 (2003)
- Ghuloum, Abdulaziz. 2006. An Incremental Approach To Compiler Construction. In Proceedings of the 2006 Scheme and Functional Programming Workshop.
- Ierusalimschy, Robert and de Figueiredo, Luiz Henrique and Celes Waldemar. The Implementation of Lua 5.0. https://www.lua.org/doc/jucs05.pdf
- Pratt, Vaughan R. 1973. Top Down Operator Precedence. Massachusetts Institute of Technology.
- Romer, Theodore H. and Lee, Dennis and Voelker, Geoffrey M. and Wolman, Alec and Wong, Wayne A. and Baer, Jean-Loup and Bershad, Brian N. and Levy, Henry M.. 1996. The Structure and Performance of Interpreters. In ASPLOS VII Proceedings of the seventh international conference on Architectural support for program- ming languages and operating systems.
- Dybvig, R. Kent. 2006. The Development of Chez Scheme. In ACM ICFP ’06

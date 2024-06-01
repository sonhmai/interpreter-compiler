# Parser

Structure
``` 
Program
    Statements
        Statement1
            Operator
            Identifier
            Expression
        Statement2
            ...
```

AST Nodes
- Program
- Statement
  - LetStatement
  - ConstStatement
  - ReturnStatement
- Operator
- Identifier
- Expression
  - IntegerLiteralExpression
  - BooleanExpression
  - InfixExpression
  - PrefixExpression
  - PostfixExpression


Pratt Parser
- to parse an expression, each operator has <= 2 parsing functions depending on type
  - infix `5 + 5`
  - prefix `--x`

Examples
- prefix expression
- infix expression
- let statement
- return statement
- function

### Expression

#### String literal expression

#### Prefix expression
``` 
-5;

tokens = [
    Operator("-"),
    IntegerLiteral(5)
]
```

#### Infix expression
``` 
1 + 2;

tokens = [
    Value(1),
    OperatorInfix("+"),
    Value(2)
]

ast = InfixExpression(
    operator="plus",
    left=IntegerLiteral(1),
    right=IntegerLiteral(2)
)

```

### Statement

#### let statement
``` 
let x = 5;
let y = 3;
let z = x + y;

tokens = [
    Keyword("let"),
    Identifier("x"),
    OperatorInfix("="),
    Value(5)
]

ast = Statement(
)
```













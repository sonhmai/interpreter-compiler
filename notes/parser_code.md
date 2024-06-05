# Parser Code

We use `Enum` in rust to represent different subtypes of a AST node because
it offers pattern matching.

In languages with interfaces like Go, interface is used instead of enum.

Why the difference?

Why we need both `current token` and `next token` for the parser? For example with `5;`
- current token is Int(5)
- we need next token `;` to tell us whether we are at end of line or start of an expression e.g. `5+6`

```rust
// how to require a common methods for all enums of Node e.g. fn node_token(&self) -> string
enum Node {
    Program {
        statements: &[Statement]
    },
    Statement(Statement),
    Expression(Expression),
}

enum Statement {
    LetStatement,
    ConstStatement,
    ReturnStatement,
}

enum Expression {
    Identifier {
        name: &str
    },
    BooleanExpression,
    IntegerLiteralExpression,
}
```

```go
package main

type Parser struct {
}

func (p *Parser) ParseProgram() *Program {
	return nil
}

type Node interface {
	NodeToken() string // first string token of this node
}

type Statement interface {
	Node
}

type Expression interface {
	Node
}

type Program struct {
	Statements []Statement
}

// NodeToken implements Node interface for Program
func (p *Program) NodeToken() string {
    return "Program"
}

type Identifier struct {
    Value string // "x" in let x=5;
}

type LetStatement struct {
	// with let x = 5;
	Name *Identifier // Identifier("x")
	Value Expression // IntegerLiteralExpression(5)
}
```

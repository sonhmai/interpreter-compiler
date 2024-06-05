# Parser Code

We use `Enum` in rust to represent different subtypes of a AST node because
it offers pattern matching.

In languages with interfaces like Go, interface is used instead of enum.

Why the difference?

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
package ast

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

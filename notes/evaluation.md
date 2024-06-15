# Interpreter - Evaluation

```pseudocode
# node: AstNode
function eval(node): 
    match node:
        case BooleanExpression: node.value
        case IntegerLiteralExpression: node.value
        case InfixExpression: eval_index(node)

function eval_infx(node):
    left = eval(node.left)
    right = eval(node.right)
    
    match node.operator:
        case "+": return left + right
        case "-": return left - right
        case ">": return left > right
        case "<": return left < right
        case _: throw Error("invalid infix expression operator")
```

``` 
1 + 2;

ast = Program(
  statements=[
    InfixExpression(
      operator="+",
      left=IntegerLiteralExpression(1),
      right=IntegerLiteralExpression(2)
    )
  ]
)

1 > 2;

ast = Program(
  statements=[
    InfixExpression(
      operator=">",
      left=IntegerLiteralExpression(1),
      right=IntegerLiteralExpression(2)
    )
  ]
)
```

what does eval return? 
= what is the interpreter `internal object system`?
= what is the value representation?

many different choices for `value representation` in interpreted language
- values represented as native types (int, bool, etc.) of the language interpreter written in.
- by a mix of pointers and native types.
- by only pointers e.g. `everything is Object` -> used here.

Evaluation with Tree-walking interpreter
- object system
- expressions evaluation `func Eval(node ast.Node) object.Object`
    - each ast node needs different evaluation which is implemented in this Eval function.
    - literals = self-evaluating expressions
        - boolean
        - integer
        - null
        - prefix (unary operator) expression e.g. `! -`
        - infix expression e.g. `+ - * / > < == !=`
    - conditionals
    - error handling
    - bindings (`let` statement, identifier evaluation)
    - functions & function calls
    - garbage collector: reusing Go's

``` 
//  evaluating expressions
Eval(*ast.Program)
-> Eval(*ast.Statement) for each Statement in *ast.Program.Statements
```

`Environment`
- keeping track of which name is mapped to which value e.g. `let x=5;`
- HashMap[string, Object]
- get(name: string) -> Option[Object]
- set(name: string, obj: Object) -> Result<()>
- pass to Eval `eval(ast.Node, Environment) -> Object`


References
- [Wren Programming Language, has two types of value representation](https://github.com/wren-lang/wren.git)
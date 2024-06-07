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
        case "+": return left+right
        case "-": return left-right
        case _: throw Error("invalid infix expression operator")
```

what does eval return? = what is the interpreter `internal object system`?

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

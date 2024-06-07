# Interpreter - Evaluation

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

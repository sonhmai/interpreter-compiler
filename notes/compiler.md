## Compiler
```
Source Code (String)
--> Lexer
Tokens
-->Parser
AST
--> Optimizer (Compiler)
Internal Representation (Bytecode)
--> Code Generator (Virtual Machine)
Machine Code (Objects)
```

Data Structures
1. bytecode
2. instruction
3. vm
- struct `Frame` (stack frame): data structure holding execution-relevant info.
- struct `VM`: virtual machine holding (1) constant pool, (2) stack,
  a stack pointer pointing to next free slot in the stack.


![img.png](../imgs/compiler_dataflow.png)


Compile Time vs Run Time

![img_1.png](../imgs/compiletime_vs_runtime.png)

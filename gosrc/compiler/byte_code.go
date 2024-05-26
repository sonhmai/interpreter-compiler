package compiler

// Bytecode contains Instructions our Compiler generated and Constants
// Compiler evaluated
type ByteCode struct {
	Instructions code.Instructions
	Constants    []object.Object
}

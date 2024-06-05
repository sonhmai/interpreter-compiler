package parser

import "testing"

func TestLetStatements(t *testing.T) {
	input := `
let x = 5;
let y = 10;
let foobar = 42;
`
	l = lexer.New(input)
	p := New(l)
}

package lexer

type Lexer struct {
	input string
}

func (l *Lexer) New(input string) *Lexer {
	return &Lexer{input: input}
}

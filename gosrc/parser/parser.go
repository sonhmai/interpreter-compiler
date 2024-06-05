package parser

type Parser struct {
	lexer Lexer
}

func (p *Parser) New(lexer Lexer) *Parser {
	return &Parser {
		lexer: lexer,
	}
}

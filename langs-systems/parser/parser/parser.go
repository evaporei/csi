package parser

import (
	e "github.com/evaporei/interpreter/expr"
	t "github.com/evaporei/interpreter/token"
)

/*
 * expression -> binary
 * binary     -> unary ( ( "AND" | "OR" ) unary )*
 * unary      -> "NOT" unary | term
 * term       -> STRING
 */

type Parser struct {
    tokens []*t.Token
    current int
}

func New(tokens []*t.Token) *Parser {
    current := 0
    return &Parser {
        tokens,
        current,
    }
}

func (p *Parser) expression() e.Expr {
    return p.binary()
}

func (p *Parser) binary() e.Expr {
    expr := p.unary()

    for p.match(t.And, t.Or) {
        op := p.previous()
        rhs := p.unary()
        expr = e.Binary{
            Lhs: expr,
            Op: op,
            Rhs: rhs,
        }
    }

    return expr
}

func (p *Parser) unary() e.Expr {
    if p.match(t.Not) {
        op := p.previous()
        rhs := p.unary()
        return e.Unary{
            Op: op,
            Rhs: rhs,
        }
    }

    return p.term()
}

func (p *Parser) term() e.Expr {
    if p.match(t.Term) {
        return e.Term{
            Value: p.previous().Literal,
        }
    }

    panic("unreachable")
}

func (p *Parser) match(types ...t.TokenType) bool {
    for _, typ := range types {
        if p.check(typ) {
            p.advance()
            return true
        }
    }

    return false
}

func (p *Parser) check(typ t.TokenType) bool {
    if p.isAtEnd() {
        return false
    }
    return p.peek().Typ == typ
}

func (p *Parser) advance() *t.Token {
    if !p.isAtEnd() {
        p.current += 1
    }
    return p.previous()
}

func (p *Parser) isAtEnd() bool {
    return p.peek().Typ == t.Eof
}

func (p *Parser) peek() *t.Token {
    return p.tokens[p.current]
}

func (p *Parser) previous() *t.Token {
    return p.tokens[p.current - 1]
}

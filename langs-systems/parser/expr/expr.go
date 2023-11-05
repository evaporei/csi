package expr

import t "github.com/evaporei/interpreter/token"

/*
/* expression -> term
 *             | unary
 *             | binary
 * unary      -> "NOT" expression
 * binary     -> expression operator expression
 * operator   -> "AND" | "OR"
 * term       -> STRING
 */

type Expr interface {
    Visit() *t.Token
}

type Unary struct {
    Op *t.Token
    Rhs Expr
}

type Binary struct {
    Lhs Expr
    Op *t.Token
    Rhs Expr
}

type Term struct {
    Value any
}

type Visitor interface {
    visitUnary(unary *Unary) *t.Token
    visitBinary(binary *Binary) *t.Token
    visitTerm(term *Term) *t.Token
}

func (u *Unary) Visit() *t.Token {
    return u.visitUnary(u)
}

func (u *Unary) visitUnary(unary *Unary) *t.Token {
}

func (b *Binary) Visit() *t.Token {
    return b.visitBinary(b)
}

func (b *Binary) visitBinary(binary *Binary) *t.Token {
}

func (self *Term) Visit() *t.Token {
    return self.visitTerm(self)
}

func (self *Term) visitTerm(term *Term) *t.Token {
}

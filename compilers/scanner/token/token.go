package token

import "fmt"

// hello AND world OR alice AND NOT bob

// OR(
//     AND(
//         TERM(hello),
//         TERM(world)),
//     AND(
//         TERM(alice),
//         NOT(
//             TERM(BOB)
//         ))

type TokenType int

const (
  // Literals
  Term TokenType = iota

  // Keywords
  And
  Or
  Not

  Eof
)

func (tt TokenType) String() string {
    switch tt {
    case Term:
        return "TERM"
    case And:
        return "AND"
    case Or:
        return "OR"
    case Not:
        return "NOT"
    }
    panic("tried to convert non-TokenType to string")
}

type Token struct {
    typ TokenType
    lexeme string
    literal any
    line int
}

func New(typ TokenType, lexeme string, literal any, line int) *Token {
    return &Token {
        typ,
        lexeme,
        literal,
        line,
    }
}

func (t *Token) String() string {
    return fmt.Sprintf("%s %s %s", t.typ.String(), t.lexeme, t.literal)
}

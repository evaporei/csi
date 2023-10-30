package scanner

import (
    t "github.com/evaporei/interpreter/token"
    e "github.com/evaporei/interpreter/error"
)

type Scanner struct {
    src string
    tokens []*t.Token

    start, curr, line int
}

func New(src string) *Scanner {
    var tokens []*t.Token
    start := 0
    curr := 0
    line := 1

    return &Scanner {
        src,
        tokens,
        start,
        curr,
        line,
    }
}

func (s *Scanner) ScanTokens() []*t.Token {
    for !s.isAtEnd() {
        s.start = s.curr
        s.scanToken()
    }

    s.tokens = append(s.tokens, t.New(t.Eof, "", nil, s.line))
    return s.tokens
}

func (s *Scanner) isAtEnd() bool {
    return s.curr >= len(s.src)
}

func (s *Scanner) scanToken() {
    c := s.advance()
    switch c {
    default:
        e.Fail(s.line, "Unexpected character.")
    }
}

func (s *Scanner) advance() rune {
    currChar := []rune(s.src)[s.curr]
    s.curr += 1
    return currChar
}

func (s *Scanner) addToken(typ t.TokenType, literal any) {
    text := string([]rune(s.src)[s.start:s.curr])
    s.tokens = append(s.tokens, t.New(typ, text, literal, s.line))
}

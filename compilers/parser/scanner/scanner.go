package scanner

import (
    t "github.com/evaporei/interpreter/token"
    e "github.com/evaporei/interpreter/error"
)

var keywords = map[string]t.TokenType{
    "AND": t.And,
    "OR": t.Or,
    "NOT": t.Not,
}

type Scanner struct {
    src []rune
    tokens []*t.Token

    start, curr, line int
}

func New(source string) *Scanner {
    src := []rune(source)
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
    case ' ':
        fallthrough
    case '\r':
        fallthrough
    case '\t':
        // do nothing
    case '\n':
        s.line += 1;
    default:
        if isAlpha(c) {
            s.term()
        } else {
            e.Fail(s.line, "Unexpected character.")
        }
    }
}

func (s *Scanner) advance() rune {
    currChar := s.src[s.curr]
    s.curr += 1
    return currChar
}

func (s *Scanner) addToken(typ t.TokenType, literal any) {
    text := string(s.src[s.start:s.curr])
    s.tokens = append(s.tokens, t.New(typ, text, literal, s.line))
}

func (s *Scanner) match(expected rune) bool {
    if s.isAtEnd() || s.src[s.curr] != expected {
        return false
    }

    s.curr += 1
    return true
}

func (s *Scanner) peek() rune {
    if s.isAtEnd() {
        return '\000'
    }
    return s.src[s.curr]
}

func (s *Scanner) peekNext() rune {
    if s.curr + 1 >= len(s.src) {
        return '\000'
    }
    return s.src[s.curr + 1]
}

func (s *Scanner) term() {
    for isAlphaNumeric(s.peek()) {
        s.advance()
    }

    text := string(s.src[s.start:s.curr])
    if typ, ok := keywords[text]; ok {
        s.addToken(typ, nil)
    } else {
        s.addToken(t.Term, nil)
    }
}

func isDigit(c rune) bool {
    return c >= '0' && c <= '9'
}

func isAlpha(c rune) bool {
    return (c >= 'a' && c <= 'z') ||
       (c >= 'A' && c <= 'Z') ||
       c == '_'
}

func isAlphaNumeric(c rune) bool {
    return isAlpha(c) || isDigit(c)
}

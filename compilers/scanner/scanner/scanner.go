package scanner

import t "github.com/evaporei/interpreter/token"

type Scanner struct {
    src string
    tokens []t.Token
}

func New(src string) *Scanner {
    var tokens []t.Token
    return &Scanner { src, tokens }
}

func (s *Scanner) ScanTokens() []t.Token {
    return s.tokens
}

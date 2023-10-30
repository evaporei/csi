package scanner

type Token struct {
}

type Scanner struct {
}

func New(src string) *Scanner {
    return &Scanner {}
}

func (s *Scanner) ScanTokens() []Token {
    var tokens []Token
    return tokens
}

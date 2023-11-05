package main

import (
	"go/parser"
	"testing"
)

func TestEvaluate(t *testing.T) {
	for _, test := range []struct {
		s        string
		expected int
	}{
		{"5", 5},
		{"1 + 1", 2},
		{"1 + 2 - 3 * 4", -9},
		{"2 * (3 - 4 * (5 + 6) + 7)", -68},
	} {
		expr, err := parser.ParseExpr(test.s)
		if err != nil {
			t.Fatal(err)
		}
		result, err := Evaluate(expr)
		if err != nil {
			t.Fatal(err)
		}
		if result != test.expected {
			t.Fatalf("Expected %q to evaluate to %d, got %d", test.s, test.expected, result)
		}
	}
}
